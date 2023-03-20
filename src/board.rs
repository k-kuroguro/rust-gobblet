use std::result;

use crate::{
   bitboard::{BitBoard, EMPTY, LINE_MASKS},
   color::{Color, ALL_COLORS, COLOR_NUM},
   error::Error,
   piece::{Piece, ALL_PIECES, PIECE_NUM},
   square::Square,
};

type Result = result::Result<Board, Error>;

#[derive(Clone, Copy, Debug)]
pub struct Board {
   bitboards: [BitBoard; 2 * PIECE_NUM], // [Black_Tiny, ..., Black_Big, White_Tiny, ..., White_Big]
   color_combined: [BitBoard; COLOR_NUM], // [Black, White]
   combined: BitBoard,
}

impl Board {
   pub fn new() -> Self {
      Self {
         bitboards: [EMPTY; 2 * PIECE_NUM],
         color_combined: [EMPTY; COLOR_NUM],
         combined: EMPTY,
      }
   }

   pub fn pieces(&self, square: Square) -> Vec<(Color, Piece)> {
      let mut result = Vec::new();
      for (i, bitboard) in self.bitboards.iter().enumerate() {
         if bitboard & square != EMPTY {
            let color = if i <= 3 { Color::Black } else { Color::White };
            let piece = match i % 4 {
               0 => Piece::Tiny,
               1 => Piece::Small,
               2 => Piece::Medium,
               3 => Piece::Big,
               _ => unreachable!(),
            };
            result.push((color, piece));
         }
      }
      result.sort_by(|a, b| (a.1 as u8).cmp(&(b.1 as u8)));
      result
   }

   pub fn place(&self, color: Color, piece: Piece, to: Square) -> Result {
      let mut board = *self;
      if !board.can_place(color, piece, to) {
         return Err(Error::InvalidPlacing(to));
      }
      board.set(color, piece, to);
      Ok(board)
   }

   pub fn r#move(&self, from: Square, to: Square) -> Result {
      let mut board = *self;
      let (color, piece) = match board.get_top(from) {
         Some(result) => result,
         None => {
            return Err(Error::InvalidMoving { from, to });
         }
      };

      if !self.can_move(color, piece, to) {
         return Err(Error::InvalidMoving { from, to });
      }

      board.unset(color, piece, from);
      board.set(color, piece, to);

      Ok(board)
   }

   pub fn can_place(&self, color: Color, piece: Piece, square: Square) -> bool {
      if self.combined & square == EMPTY {
         return true;
      }

      if let Piece::Tiny = piece {
         return false;
      }
      if self.color_combined[color.reverse() as usize] & square != EMPTY {
         if self.has_3_in_a_row(color.reverse(), square) {
            for x in (piece as usize..=Piece::Big as usize).rev() {
               if self.bitboards[x + 4 * color.reverse() as usize] & square != EMPTY {
                  return false;
               }
            }
            return true;
         }
         return false;
      } else {
         for x in (piece as usize..=Piece::Big as usize).rev() {
            if self.bitboards[x + 4 * color as usize] & square != EMPTY {
               return false;
            }
         }
         return true;
      }
   }

   pub fn can_move(&self, color: Color, piece: Piece, to: Square) -> bool {
      if self.combined & to == EMPTY {
         return true;
      }

      if let Piece::Tiny = piece {
         return false;
      }
      if self.color_combined[color.reverse() as usize] & to != EMPTY {
         for x in (piece as usize..=Piece::Big as usize).rev() {
            if self.bitboards[x + 4 * color.reverse() as usize] & to != EMPTY {
               return false;
            }
         }
         return true;
      } else {
         for x in (piece as usize..=Piece::Big as usize).rev() {
            if self.bitboards[x + 4 * color as usize] & to != EMPTY {
               return false;
            }
         }
         return true;
      }
   }

   pub fn has_won(&self, color: Color) -> bool {
      let color_combined = &self.color_combined[color as usize];
      for mask in LINE_MASKS {
         if color_combined & mask == mask {
            return true;
         }
      }
      false
   }

   fn set(&mut self, color: Color, piece: Piece, square: Square) {
      self.bitboards[piece as usize + 4 * color as usize] |= square;
      self.combine();
   }

   fn unset(&mut self, color: Color, piece: Piece, square: Square) {
      self.bitboards[piece as usize + 4 * color as usize] &= !BitBoard::from_square(square);
      self.combine();
   }

   fn combine(&mut self) {
      self.color_combined = ALL_COLORS.map(|color| {
         let mut result = self.bitboards[Piece::Big as usize + 4 * color as usize];
         for piece in &ALL_PIECES[1..] {
            let bigger = (*piece as usize + 1..=Piece::Big as usize).fold(EMPTY, |acc, piece| {
               acc | self.bitboards[piece + 4 * color.reverse() as usize]
            });
            result |= !bigger & self.bitboards[*piece as usize + 4 * color as usize];
         }
         result
      });
      self.combined = self.color_combined[0] | self.color_combined[1];
   }

   fn get_top(&self, square: Square) -> Option<(Color, Piece)> {
      if self.combined & square == EMPTY {
         return None;
      }

      let color = if self.color_combined[Color::Black as usize] & square != EMPTY {
         Color::Black
      } else {
         Color::White
      };
      let piece = {
         let mut result = Piece::Tiny;
         for piece in ALL_PIECES {
            if self.bitboards[piece as usize + 4 * color as usize] & square != EMPTY {
               result = piece;
            }
         }
         result
      };

      Some((color, piece))
   }

   fn has_3_in_a_row(&self, color: Color, square: Square) -> bool {
      let color_combined = &self.color_combined[color as usize];
      for mask in LINE_MASKS {
         if mask & square != EMPTY && (color_combined & mask).popcount() == 3 {
            return true;
         }
      }
      false
   }
}

#[cfg(test)]
mod tests {
   use crate::{
      bitboard::BitBoard, board::Board, color::Color, error::Error, piece::Piece, square::Square,
   };

   #[test]
   fn test_place() {
      let mut board = Board::new();
      let mut error;

      // Place on empty square.
      board = board.place(Color::Black, Piece::Big, Square::C2).unwrap();
      board = board.place(Color::White, Piece::Small, Square::B3).unwrap();
      assert_eq!(
         board.bitboards,
         [
            BitBoard::new(0x0000),
            BitBoard::new(0x0000),
            BitBoard::new(0x0000),
            BitBoard::new(0x0200),
            BitBoard::new(0x0000),
            BitBoard::new(0x0040),
            BitBoard::new(0x0000),
            BitBoard::new(0x0000)
         ]
      );
      assert_eq!(
         board.color_combined,
         [BitBoard::new(0x0200), BitBoard::new(0x0040)]
      );
      assert_eq!(board.combined, BitBoard::new(0x0240));

      // Place on same colored piece.
      board = board
         .place(Color::White, Piece::Medium, Square::B3)
         .unwrap();
      assert_eq!(
         board.bitboards,
         [
            BitBoard::new(0x0000),
            BitBoard::new(0x0000),
            BitBoard::new(0x0000),
            BitBoard::new(0x0200),
            BitBoard::new(0x0000),
            BitBoard::new(0x0040),
            BitBoard::new(0x0040),
            BitBoard::new(0x0000)
         ]
      );
      assert_eq!(
         board.color_combined,
         [BitBoard::new(0x0200), BitBoard::new(0x0040)]
      );
      assert_eq!(board.combined, BitBoard::new(0x0240));

      // Place on different colored piece.
      error = board.place(Color::Black, Piece::Big, Square::B3);
      assert!(error.is_err());

      // Make a line.
      board = board.place(Color::Black, Piece::Tiny, Square::A3).unwrap();
      board = board.place(Color::Black, Piece::Tiny, Square::C3).unwrap();
      board = board.place(Color::Black, Piece::Tiny, Square::D3).unwrap();

      // Place on different colored piece.
      board = board.place(Color::White, Piece::Small, Square::A3).unwrap();
      assert_eq!(
         board.bitboards,
         [
            BitBoard::new(0x00B0),
            BitBoard::new(0x0000),
            BitBoard::new(0x0000),
            BitBoard::new(0x0200),
            BitBoard::new(0x0000),
            BitBoard::new(0x00C0),
            BitBoard::new(0x0040),
            BitBoard::new(0x0000)
         ]
      );
      assert_eq!(
         board.color_combined,
         [BitBoard::new(0x0230), BitBoard::new(0x00C0)]
      );
      assert_eq!(board.combined, BitBoard::new(0x02F0));

      // Place on bigger size piece.
      error = board.place(Color::White, Piece::Medium, Square::B3);
      assert!(error.is_err());
      error = board.place(Color::Black, Piece::Medium, Square::C2);
      assert!(error.is_err());
   }

   #[test]
   fn test_move() {
      let mut board = Board::new();
      let mut error;

      // Move from empty square.
      error = board.r#move(Square::A1, Square::A2);
      assert!(error.is_err());

      // Move to empty square.
      board = board.place(Color::Black, Piece::Big, Square::A1).unwrap();
      board = board.r#move(Square::A1, Square::A2).unwrap();
      assert_eq!(
         board.bitboards,
         [
            BitBoard::new(0x0000),
            BitBoard::new(0x0000),
            BitBoard::new(0x0000),
            BitBoard::new(0x0800),
            BitBoard::new(0x0000),
            BitBoard::new(0x0000),
            BitBoard::new(0x0000),
            BitBoard::new(0x0000)
         ]
      );
      assert_eq!(
         board.color_combined,
         [BitBoard::new(0x0800), BitBoard::new(0x0000)]
      );
      assert_eq!(board.combined, BitBoard::new(0x0800));

      // Move to square with a smaller piece.
      board = board.place(Color::White, Piece::Tiny, Square::A3).unwrap();
      board = board
         .place(Color::White, Piece::Medium, Square::A3)
         .unwrap();
      board = board.r#move(Square::A2, Square::A3).unwrap();
      assert_eq!(
         board.bitboards,
         [
            BitBoard::new(0x0000),
            BitBoard::new(0x0000),
            BitBoard::new(0x0000),
            BitBoard::new(0x0080),
            BitBoard::new(0x0080),
            BitBoard::new(0x0000),
            BitBoard::new(0x0080),
            BitBoard::new(0x0000)
         ]
      );
      assert_eq!(
         board.color_combined,
         [BitBoard::new(0x0080), BitBoard::new(0x0000)]
      );
      assert_eq!(board.combined, BitBoard::new(0x0080));

      // Move to square with a bigger piece.
      board = board
         .place(Color::White, Piece::Medium, Square::C4)
         .unwrap();
      error = board.r#move(Square::C4, Square::A3);
      assert!(error.is_err());

      // Move the top of stacked piece.
      board = board.r#move(Square::A3, Square::D1).unwrap();
      assert_eq!(
         board.bitboards,
         [
            BitBoard::new(0x0000),
            BitBoard::new(0x0000),
            BitBoard::new(0x0000),
            BitBoard::new(0x1000),
            BitBoard::new(0x0080),
            BitBoard::new(0x0000),
            BitBoard::new(0x0082),
            BitBoard::new(0x0000)
         ]
      );
      assert_eq!(
         board.color_combined,
         [BitBoard::new(0x1000), BitBoard::new(0x0082)]
      );
      assert_eq!(board.combined, BitBoard::new(0x1082));
   }
}
