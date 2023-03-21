use crate::{board::Board, color::Color, error::Error, hand::Hand, square::Square};

#[derive(Clone, Copy, Debug)]
pub enum Action {
   Move { from: Square, to: Square },
   PlaceFromHand { index: usize, to: Square },
}

#[derive(Clone, Copy, Debug)]
pub enum Status {
   OnGoing,
   BlackWins,
   WhiteWins,
}

#[derive(Clone, Debug)]
pub struct Game {
   board: Board,
   hands: [Hand; 2], // [Black, White]
   turn: Color,
   status: Status,
}

impl Game {
   pub fn new() -> Self {
      Self {
         board: Board::new(),
         hands: [Hand::new(), Hand::new()],
         turn: Color::Black,
         status: Status::OnGoing,
      }
   }

   pub fn board(&self) -> &Board {
      &self.board
   }

   pub fn hand(&self, color: Color) -> &Hand {
      &self.hands[color as usize]
   }

   pub fn turn(&self) -> &Color {
      &self.turn
   }

   pub fn status(&self) -> &Status {
      &self.status
   }

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
            if let Some(piece) = self.hands[self.turn as usize].pop(index) {
               match self.board.place(self.turn, piece, to) {
                  Ok(board) => {
                     self.board = board;
                  }
                  Err(err) => {
                     return Err(err);
                  }
               };
            } else {
               return Err(Error::EmptyHand);
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
