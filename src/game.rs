use crate::{board::Board, color::Color, error::Error, hand::Hand, square::Square};

//TODO: 引き分け

#[derive(Clone, Copy, Debug)]
pub enum Action {
   Move { from: Square, to: Square }, //TODO: Move 構造体
   PlaceFromHand { index: usize, to: Square }, //TODO: インデックスアクセスは安全でない.
                                      //TODO: 降参
}

#[derive(Clone, Copy, Debug)]
pub enum Status {
   OnGoing,
   BlackWins,
   WhiteWins,
   //TODO: 引き分け
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

      let mut result = Ok(Status::OnGoing);
      match action {
         Action::Move { from, to } => match self.board.r#move(from, to) {
            Ok(board) => {
               self.board = board;
            }
            Err(err) => {
               result = Err(err);
            }
         },
         Action::PlaceFromHand { index, to } => {
            if let Some(piece) = self.hands[self.turn as usize].pop(index) {
               match self.board.place(self.turn, piece, to) {
                  Ok(board) => {
                     self.board = board;
                  }
                  Err(err) => {
                     result = Err(err);
                  }
               };
            } else {
               //TODO: Err
            }
         }
      };
      if self.board.has_won(Color::Black) {
         self.status = Status::BlackWins; //TODO: どっちかにしたら?
         result = Ok(Status::BlackWins);
      }
      if self.board.has_won(Color::White) {
         self.status = Status::WhiteWins;
         result = Ok(Status::WhiteWins);
      }
      self.change_turn();
      result
   }

   fn change_turn(&mut self) {
      self.turn = self.turn.reverse();
   }
}
