use std::{result, vec};

use crate::{
   bitboard::{BitBoard, EMPTY, LINE_MASKS},
   color::Color,
   error::Error,
   piece::{Piece, PieceKind, PieceSet},
   square::Square,
};

type Result = result::Result<Board, Error>;

#[derive(Clone, Copy, Debug)]
pub struct Board {
   bitboards: [BitBoard; 2 * PieceKind::NUM], // [Black_Tiny, ..., Black_Big, White_Tiny, ..., White_Big]
   color_combined: [BitBoard; Color::NUM],    // [Black, White]
   combined: BitBoard,
}

impl Board {
   pub const SIZE: usize = 4;

   pub fn new() -> Self {
      Self {
         bitboards: [EMPTY; 2 * PieceKind::NUM],
         color_combined: [EMPTY; Color::NUM],
         combined: EMPTY,
      }
   }

   pub fn exists(&self, square: Square) -> bool {
      self.combined & square != EMPTY
   }

   pub fn place(&self, piece: Piece, to: Square) -> Result {
      let mut board = *self;
      if !board.can_place(piece, to) {
         return Err(Error::InvalidPlacing(to));
      }
      board.set(piece, to);
      Ok(board)
   }

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

   pub fn can_place(&self, piece: Piece, to: Square) -> bool {
      if self.combined & to == EMPTY {
         return true;
      }

      let Piece { color, kind } = piece;

      if let PieceKind::Tiny = kind {
         return false;
      }
      if self.color_combined[color.reverse() as usize] & to != EMPTY {
         if self.has_3_in_a_row(color.reverse(), to) {
            for x in (kind as usize..=PieceKind::Big as usize).rev() {
               if self.bitboards[x + 4 * color.reverse() as usize] & to != EMPTY {
                  return false;
               }
            }
            return true;
         }
         return false;
      } else {
         for x in (kind as usize..=PieceKind::Big as usize).rev() {
            if self.bitboards[x + 4 * color as usize] & to != EMPTY {
               return false;
            }
         }
         return true;
      }
   }

   pub fn can_move(&self, piece: Piece, to: Square) -> bool {
      if self.combined & to == EMPTY {
         return true;
      }

      let Piece { color, kind } = piece;

      if let PieceKind::Tiny = kind {
         return false;
      }
      if self.color_combined[color.reverse() as usize] & to != EMPTY {
         for x in (kind as usize..=PieceKind::Big as usize).rev() {
            if self.bitboards[x + 4 * color.reverse() as usize] & to != EMPTY {
               return false;
            }
         }
         return true;
      } else {
         for x in (kind as usize..=PieceKind::Big as usize).rev() {
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

   fn set(&mut self, piece: Piece, square: Square) {
      let Piece { color, kind } = piece;
      self.bitboards[kind as usize + 4 * color as usize] |= square;
      self.combine();
   }

   fn unset(&mut self, piece: Piece, square: Square) {
      let Piece { color, kind } = piece;
      self.bitboards[kind as usize + 4 * color as usize] &= !BitBoard::from_square(square);
      self.combine();
   }

   fn combine(&mut self) {
      self.color_combined = Color::ALL.map(|color| {
         let mut result = self.bitboards[PieceKind::Big as usize + 4 * color as usize];
         for kind in &PieceKind::ALL[1..] {
            let bigger = (*kind as usize + 1..=PieceKind::Big as usize).fold(EMPTY, |acc, kind| {
               acc | self.bitboards[kind + 4 * color.reverse() as usize]
            });
            result |= !bigger & self.bitboards[*kind as usize + 4 * color as usize];
         }
         result
      });
      self.combined = self.color_combined[0] | self.color_combined[1];
   }

   fn get_top(&self, square: Square) -> Option<Piece> {
      if self.combined & square == EMPTY {
         return None;
      }

      let color = if self.color_combined[Color::Black as usize] & square != EMPTY {
         Color::Black
      } else {
         Color::White
      };
      let kind = {
         let mut result = PieceKind::Tiny;
         for kind in PieceKind::ALL {
            if self.bitboards[kind as usize + 4 * color as usize] & square != EMPTY {
               result = kind;
            }
         }
         result
      };

      Some(Piece::new(color, kind))
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
                  if bitboard & square != EMPTY {
                     let color = if i <= 3 { Color::Black } else { Color::White };
                     let piece = match i % 4 {
                        0 => PieceKind::Tiny,
                        1 => PieceKind::Small,
                        2 => PieceKind::Medium,
                        3 => PieceKind::Big,
                        _ => unreachable!(),
                     };
                     pieces.push(Piece::new(color, piece));
                  }
               }
               pieces.sort_by(|a, b| (a.kind as u8).cmp(&(b.kind as u8)));
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
      piece::{Piece, PieceKind, PieceSet},
      square::Square,
   };

   #[test]
   fn test_place() {
      let mut board = Board::new();
      let mut error;

      // Place on empty square.
      board = board
         .place(Piece::new(Color::Black, PieceKind::Big), Square::C2)
         .unwrap();
      board = board
         .place(Piece::new(Color::White, PieceKind::Small), Square::B3)
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
         .place(Piece::new(Color::White, PieceKind::Medium), Square::B3)
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
      error = board.place(Piece::new(Color::Black, PieceKind::Big), Square::B3);
      assert!(error.is_err());

      // Make a line.
      board = board
         .place(Piece::new(Color::Black, PieceKind::Tiny), Square::A3)
         .unwrap();
      board = board
         .place(Piece::new(Color::Black, PieceKind::Tiny), Square::C3)
         .unwrap();
      board = board
         .place(Piece::new(Color::Black, PieceKind::Tiny), Square::D3)
         .unwrap();

      // Place on different colored piece.
      board = board
         .place(Piece::new(Color::White, PieceKind::Small), Square::A3)
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
      error = board.place(Piece::new(Color::White, PieceKind::Medium), Square::B3);
      assert!(error.is_err());
      error = board.place(Piece::new(Color::Black, PieceKind::Medium), Square::C2);
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
         .place(Piece::new(Color::Black, PieceKind::Big), Square::A1)
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
         .place(Piece::new(Color::White, PieceKind::Tiny), Square::A3)
         .unwrap();
      board = board
         .place(Piece::new(Color::White, PieceKind::Medium), Square::A3)
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
         .place(Piece::new(Color::White, PieceKind::Medium), Square::C4)
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
         .place(Piece::new(Color::White, PieceKind::Tiny), Square::A3)
         .unwrap();
      board = board
         .place(Piece::new(Color::Black, PieceKind::Medium), Square::A2)
         .unwrap();
      board = board.r#move(Square::A2, Square::A3).unwrap();
      board = board
         .place(Piece::new(Color::White, PieceKind::Big), Square::A2)
         .unwrap();
      board = board.r#move(Square::A2, Square::A3).unwrap();
      board = board
         .place(Piece::new(Color::Black, PieceKind::Big), Square::C1)
         .unwrap();
      board = board
         .place(Piece::new(Color::White, PieceKind::Small), Square::B4)
         .unwrap();

      for (i, set) in board.into_iter().enumerate() {
         match i {
            2 => {
               assert_eq!(
                  set,
                  PieceSet::from_slice(&[Piece::new(Color::Black, PieceKind::Big)])
               )
            }
            8 => {
               assert_eq!(
                  set,
                  PieceSet::from_slice(&[
                     Piece::new(Color::White, PieceKind::Tiny),
                     Piece::new(Color::Black, PieceKind::Medium),
                     Piece::new(Color::White, PieceKind::Big)
                  ])
               )
            }
            13 => {
               assert_eq!(
                  set,
                  PieceSet::from_slice(&[Piece::new(Color::White, PieceKind::Small)])
               )
            }
            _ => {
               assert_eq!(set, PieceSet::none());
            }
         }
      }
   }
}
