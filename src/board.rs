use std::{result, vec};

use crate::{BitBoard, Color, Error, Piece, PieceSet, Size, Square};

type Result = result::Result<Board, Error>;

/// A representation of a Gobblet board.
#[derive(Clone, Copy, Debug)]
pub struct Board {
   bitboards: [BitBoard; 2 * Size::NUM], // [Black_Tiny, ..., Black_Big, White_Tiny, ..., White_Big]
   color_combined: [BitBoard; Color::NUM], // [Black, White]
   combined: BitBoard,
}

impl Board {
   pub const SIZE: usize = 4;

   pub fn new() -> Self {
      Self {
         bitboards: [BitBoard::EMPTY; 2 * Size::NUM],
         color_combined: [BitBoard::EMPTY; Color::NUM],
         combined: BitBoard::EMPTY,
      }
   }

   /// Returns a bitboard with only the given piece.
   pub fn pieces(&self, piece: Piece) -> BitBoard {
      self.bitboards[piece.size as usize + 4 * piece.color as usize]
   }

   /// Returns a bitboard viewed from above with only the given color.
   /// If a piece of a different color is stacked on top, the bit will be 0.
   pub fn color_combined(&self, color: Color) -> BitBoard {
      self.color_combined[color as usize]
   }

   /// Returns a bitboard viewed from above.
   pub fn combined(&self) -> BitBoard {
      self.combined
   }

   /// Checks if any piece exists on the given square.
   pub fn exists(&self, square: Square) -> bool {
      self.combined & square != BitBoard::EMPTY
   }

   /// Places a piece to the given square and returns the new board.
   pub fn place(&self, piece: Piece, to: Square) -> Result {
      let mut board = *self;
      if !board.can_place(piece, to) {
         return Err(Error::InvalidPlacing(to));
      }
      board.set(piece, to);
      Ok(board)
   }

   /// Moves a piece from one square to another and returns the new board.
   pub fn r#move(&self, from: Square, to: Square) -> Result {
      let mut board = *self;
      let piece = match board.get_top(from) {
         Some(result) => result,
         None => {
            return Err(Error::InvalidMoving { from, to });
         }
      };

      if !self.can_move(piece, to) {
         return Err(Error::InvalidMoving { from, to });
      }

      board.unset(piece, from);
      board.set(piece, to);

      Ok(board)
   }

   /// Checks if a piece can be placed on the given square.
   pub fn can_place(&self, piece: Piece, to: Square) -> bool {
      if self.combined & to == BitBoard::EMPTY {
         return true;
      }

      let Piece { color, size } = piece;

      if self.color_combined[color.reverse() as usize] & to != BitBoard::EMPTY {
         if self.has_3_in_a_row(color.reverse(), to) {
            size > self.get_top(to).unwrap().size
         } else {
            false
         }
      } else {
         size > self.get_top(to).unwrap().size
      }
   }

   /// Checks if a piece can be moved to the given square.
   pub fn can_move(&self, piece: Piece, to: Square) -> bool {
      if let Some(top) = self.get_top(to) {
         piece.size > top.size
      } else {
         true
      }
   }

   /// Checks if a player of the given color has won the game.
   pub fn has_won(&self, color: Color) -> bool {
      let color_combined = &self.color_combined[color as usize];
      for mask in BitBoard::LINE_MASKS {
         if color_combined & mask == mask {
            return true;
         }
      }
      false
   }

   /// Returns the top piece on the given square, or None if there is no piece.
   pub fn get_top(&self, square: Square) -> Option<Piece> {
      if self.combined & square == BitBoard::EMPTY {
         return None;
      }

      let color = if self.color_combined[Color::Black as usize] & square != BitBoard::EMPTY {
         Color::Black
      } else {
         Color::White
      };
      let size = {
         let mut result = Size::Tiny;
         for size in Size::ALL {
            if self.pieces(Piece::new(color, size)) & square != BitBoard::EMPTY {
               result = size;
            }
         }
         result
      };

      Some(Piece::new(color, size))
   }

   fn set(&mut self, piece: Piece, square: Square) {
      let Piece { color, size } = piece;
      self.bitboards[size as usize + 4 * color as usize] |= square;
      self.combine();
   }

   fn unset(&mut self, piece: Piece, square: Square) {
      let Piece { color, size } = piece;
      self.bitboards[size as usize + 4 * color as usize] &= !BitBoard::from_square(square);
      self.combine();
   }

   fn combine(&mut self) {
      self.color_combined = Color::ALL.map(|color| {
         let mut result = self.pieces(Piece::new(color, Size::Big));
         for &size in Size::ALL[..3].iter().rev() {
            let bigger = Size::ALL[size as usize + 1..]
               .iter()
               .fold(BitBoard::EMPTY, |acc, &size| {
                  acc | self.pieces(Piece::new(color.reverse(), size))
               });
            result |= !bigger & self.pieces(Piece::new(color, size));
         }
         result
      });
      self.combined = self.color_combined[0] | self.color_combined[1];
   }

   fn has_3_in_a_row(&self, color: Color, square: Square) -> bool {
      let color_combined = &self.color_combined[color as usize];
      for mask in BitBoard::LINE_MASKS {
         if mask & square != BitBoard::EMPTY && (color_combined & mask).popcount() == 3 {
            return true;
         }
      }
      false
   }
}

macro_rules! into_iterator {
   ($type:ty) => {
      impl IntoIterator for $type {
         type Item = PieceSet;
         type IntoIter = vec::IntoIter<PieceSet>;

         fn into_iter(self) -> Self::IntoIter {
            let mut result = Vec::new();
            for square in Square::ALL {
               let mut pieces = Vec::new();
               for (i, bitboard) in self.bitboards.iter().enumerate() {
                  if bitboard & square != BitBoard::EMPTY {
                     let color = if i <= 3 { Color::Black } else { Color::White };
                     let piece = match i % 4 {
                        0 => Size::Tiny,
                        1 => Size::Small,
                        2 => Size::Medium,
                        3 => Size::Big,
                        _ => unreachable!(),
                     };
                     pieces.push(Piece::new(color, piece));
                  }
               }
               pieces.sort_by(|a, b| (a.size as u8).cmp(&(b.size as u8)));
               result.push(PieceSet::from_vec(pieces))
            }
            result.into_iter()
         }
      }
   };
}

into_iterator!(Board);
into_iterator!(&Board);

#[cfg(test)]
mod tests {
   use crate::{
      bitboard::BitBoard,
      board::Board,
      color::Color,
      piece::{Piece, PieceSet, Size},
      square::Square,
   };

   #[test]
   fn test_place() {
      let mut board = Board::new();
      let mut error;

      // Place on empty square.
      board = board
         .place(Piece::new(Color::Black, Size::Big), Square::C2)
         .unwrap();
      board = board
         .place(Piece::new(Color::White, Size::Small), Square::B3)
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
         .place(Piece::new(Color::White, Size::Medium), Square::B3)
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
      error = board.place(Piece::new(Color::Black, Size::Big), Square::B3);
      assert!(error.is_err());

      // Make a line.
      board = board
         .place(Piece::new(Color::Black, Size::Tiny), Square::A3)
         .unwrap();
      board = board
         .place(Piece::new(Color::Black, Size::Tiny), Square::C3)
         .unwrap();
      board = board
         .place(Piece::new(Color::Black, Size::Tiny), Square::D3)
         .unwrap();

      // Place on different colored piece.
      board = board
         .place(Piece::new(Color::White, Size::Small), Square::A3)
         .unwrap();
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
      error = board.place(Piece::new(Color::White, Size::Medium), Square::B3);
      assert!(error.is_err());
      error = board.place(Piece::new(Color::Black, Size::Medium), Square::C2);
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
      board = board
         .place(Piece::new(Color::Black, Size::Big), Square::A1)
         .unwrap();
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
      board = board
         .place(Piece::new(Color::White, Size::Tiny), Square::A3)
         .unwrap();
      board = board
         .place(Piece::new(Color::White, Size::Medium), Square::A3)
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
         .place(Piece::new(Color::White, Size::Medium), Square::C4)
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

   #[test]
   fn test_iter() {
      let mut board = Board::new();
      board = board
         .place(Piece::new(Color::White, Size::Tiny), Square::A3)
         .unwrap();
      board = board
         .place(Piece::new(Color::Black, Size::Medium), Square::A2)
         .unwrap();
      board = board.r#move(Square::A2, Square::A3).unwrap();
      board = board
         .place(Piece::new(Color::White, Size::Big), Square::A2)
         .unwrap();
      board = board.r#move(Square::A2, Square::A3).unwrap();
      board = board
         .place(Piece::new(Color::Black, Size::Big), Square::C1)
         .unwrap();
      board = board
         .place(Piece::new(Color::White, Size::Small), Square::B4)
         .unwrap();

      for (i, set) in board.into_iter().enumerate() {
         match i {
            2 => {
               assert_eq!(
                  set,
                  PieceSet::from_slice(&[Piece::new(Color::Black, Size::Big)])
               )
            }
            8 => {
               assert_eq!(
                  set,
                  PieceSet::from_slice(&[
                     Piece::new(Color::White, Size::Tiny),
                     Piece::new(Color::Black, Size::Medium),
                     Piece::new(Color::White, Size::Big)
                  ])
               )
            }
            13 => {
               assert_eq!(
                  set,
                  PieceSet::from_slice(&[Piece::new(Color::White, Size::Small)])
               )
            }
            _ => {
               assert_eq!(set, PieceSet::none());
            }
         }
      }
   }
}
