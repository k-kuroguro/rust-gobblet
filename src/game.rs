use crate::{Board, Color, Error, Hand, Square};

/// Represents actions that can be executed in the game.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Action {
   Move { from: Square, to: Square },
   PlaceFromHand { index: usize, to: Square },
}

/// Represents the progress or result of the game.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Status {
   OnGoing,
   BlackWins,
   WhiteWins,
}

/// Manages board, hands, turn, and result of the game.
#[derive(Clone, Debug)]
pub struct Game {
   board: Board,
   hands: [Hand; Color::NUM],
   turn: Color,
   status: Status,
}

impl Game {
   pub fn new() -> Self {
      Self {
         board: Board::new(),
         hands: [Hand::new(Color::Black), Hand::new(Color::White)],
         turn: Color::Black,
         status: Status::OnGoing,
      }
   }

   /// Returns the board.
   pub fn board(&self) -> &Board {
      &self.board
   }

   /// Returns the hand of the given color.
   pub fn hand(&self, color: Color) -> &Hand {
      &self.hands[color as usize]
   }

   /// Returns the color of the current player's turn.
   pub fn turn(&self) -> Color {
      self.turn
   }

   /// Returns the status of the game.
   pub fn status(&self) -> Status {
      self.status
   }

   /// Executes the given action and returns result.
   pub fn execute(&mut self, action: Action) -> Result<Status, Error> {
      match self.status {
         Status::OnGoing => {}
         Status::BlackWins => {
            return Ok(Status::BlackWins);
         }
         Status::WhiteWins => {
            return Ok(Status::WhiteWins);
         }
      };

      let result;
      match action {
         Action::Move { from, to } => match self.board.r#move(from, to) {
            Ok(board) => {
               self.board = board;
            }
            Err(err) => {
               return Err(err);
            }
         },
         Action::PlaceFromHand { index, to } => {
            if let Some(&piece) = self.hands[self.turn as usize].peek(index) {
               match self.board.place(piece, to) {
                  Ok(board) => {
                     self.hands[self.turn as usize].pop(index);
                     self.board = board;
                  }
                  Err(err) => {
                     return Err(err);
                  }
               };
            }
         }
      };

      if self.board.has_won(Color::Black) {
         self.status = Status::BlackWins;
      }
      if self.board.has_won(Color::White) {
         self.status = Status::WhiteWins;
      }

      self.change_turn();

      result = Ok(self.status);
      result
   }

   fn change_turn(&mut self) {
      self.turn = self.turn.reverse();
   }
}
