use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use crate::square::Square;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct BitBoard(u16);

pub const EMPTY: BitBoard = BitBoard(0);
pub const LINE_MASKS: [BitBoard; 10] = [
   // Cols
   BitBoard(0x8888),
   BitBoard(0x4444),
   BitBoard(0x2222),
   BitBoard(0x1111),
   // Rows
   BitBoard(0xF000),
   BitBoard(0x0F00),
   BitBoard(0x00F0),
   BitBoard(0x000F),
   // Diagonals
   BitBoard(0x8421),
   BitBoard(0x1248),
];

impl BitBoard {
   pub fn new(n: u16) -> Self {
      Self(n)
   }

   pub fn popcount(&self) -> u8 {
      self.0.count_ones() as u8
   }

   pub fn from_square(square: Square) -> Self {
      Self(square as u16)
   }
}

impl BitAnd for BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitand(self, other: BitBoard) -> BitBoard {
      BitBoard(self.0 & other.0)
   }
}

impl BitAnd for &BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitand(self, other: &BitBoard) -> BitBoard {
      BitBoard(self.0 & other.0)
   }
}

impl BitAnd<&BitBoard> for BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitand(self, other: &BitBoard) -> BitBoard {
      BitBoard(self.0 & other.0)
   }
}

impl BitAnd<BitBoard> for &BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitand(self, other: BitBoard) -> BitBoard {
      BitBoard(self.0 & other.0)
   }
}

impl BitAnd<Square> for BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitand(self, other: Square) -> BitBoard {
      BitBoard(self.0 & other as u16)
   }
}

impl BitAnd<Square> for &BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitand(self, other: Square) -> BitBoard {
      BitBoard(self.0 & other as u16)
   }
}

impl BitAnd<&Square> for BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitand(self, other: &Square) -> BitBoard {
      BitBoard(self.0 & *other as u16)
   }
}

impl BitAnd<&Square> for &BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitand(self, other: &Square) -> BitBoard {
      BitBoard(self.0 & *other as u16)
   }
}

impl BitOr for BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitor(self, other: BitBoard) -> BitBoard {
      BitBoard(self.0 | other.0)
   }
}

impl BitOr for &BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitor(self, other: &BitBoard) -> BitBoard {
      BitBoard(self.0 | other.0)
   }
}

impl BitOr<&BitBoard> for BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitor(self, other: &BitBoard) -> BitBoard {
      BitBoard(self.0 | other.0)
   }
}

impl BitOr<BitBoard> for &BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitor(self, other: BitBoard) -> BitBoard {
      BitBoard(self.0 | other.0)
   }
}

impl BitOr<Square> for BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitor(self, other: Square) -> BitBoard {
      BitBoard(self.0 | other as u16)
   }
}

impl BitOr<Square> for &BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitor(self, other: Square) -> BitBoard {
      BitBoard(self.0 | other as u16)
   }
}

impl BitOr<&Square> for BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitor(self, other: &Square) -> BitBoard {
      BitBoard(self.0 | *other as u16)
   }
}

impl BitOr<&Square> for &BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitor(self, other: &Square) -> BitBoard {
      BitBoard(self.0 | *other as u16)
   }
}

impl BitXor for BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitxor(self, other: BitBoard) -> BitBoard {
      BitBoard(self.0 ^ other.0)
   }
}

impl BitXor for &BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitxor(self, other: &BitBoard) -> BitBoard {
      BitBoard(self.0 ^ other.0)
   }
}

impl BitXor<&BitBoard> for BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitxor(self, other: &BitBoard) -> BitBoard {
      BitBoard(self.0 ^ other.0)
   }
}

impl BitXor<BitBoard> for &BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitxor(self, other: BitBoard) -> BitBoard {
      BitBoard(self.0 ^ other.0)
   }
}

impl BitXor<Square> for BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitxor(self, other: Square) -> BitBoard {
      BitBoard(self.0 ^ other as u16)
   }
}

impl BitXor<Square> for &BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitxor(self, other: Square) -> BitBoard {
      BitBoard(self.0 ^ other as u16)
   }
}

impl BitXor<&Square> for BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitxor(self, other: &Square) -> BitBoard {
      BitBoard(self.0 ^ *other as u16)
   }
}

impl BitXor<&Square> for &BitBoard {
   type Output = BitBoard;

   #[inline]
   fn bitxor(self, other: &Square) -> BitBoard {
      BitBoard(self.0 ^ *other as u16)
   }
}

impl BitAndAssign for BitBoard {
   #[inline]
   fn bitand_assign(&mut self, other: BitBoard) {
      self.0 &= other.0;
   }
}

impl BitAndAssign<&BitBoard> for BitBoard {
   #[inline]
   fn bitand_assign(&mut self, other: &BitBoard) {
      self.0 &= other.0;
   }
}

impl BitAndAssign<Square> for BitBoard {
   #[inline]
   fn bitand_assign(&mut self, other: Square) {
      self.0 &= other as u16;
   }
}

impl BitAndAssign<&Square> for BitBoard {
   #[inline]
   fn bitand_assign(&mut self, other: &Square) {
      self.0 &= *other as u16;
   }
}

impl BitOrAssign for BitBoard {
   #[inline]
   fn bitor_assign(&mut self, other: BitBoard) {
      self.0 |= other.0;
   }
}

impl BitOrAssign<&BitBoard> for BitBoard {
   #[inline]
   fn bitor_assign(&mut self, other: &BitBoard) {
      self.0 |= other.0;
   }
}

impl BitOrAssign<Square> for BitBoard {
   #[inline]
   fn bitor_assign(&mut self, other: Square) {
      self.0 |= other as u16;
   }
}

impl BitOrAssign<&Square> for BitBoard {
   #[inline]
   fn bitor_assign(&mut self, other: &Square) {
      self.0 |= *other as u16;
   }
}

impl BitXorAssign for BitBoard {
   #[inline]
   fn bitxor_assign(&mut self, other: BitBoard) {
      self.0 ^= other.0;
   }
}

impl BitXorAssign<&BitBoard> for BitBoard {
   #[inline]
   fn bitxor_assign(&mut self, other: &BitBoard) {
      self.0 ^= other.0;
   }
}

impl BitXorAssign<Square> for BitBoard {
   #[inline]
   fn bitxor_assign(&mut self, other: Square) {
      self.0 ^= other as u16;
   }
}

impl BitXorAssign<&Square> for BitBoard {
   #[inline]
   fn bitxor_assign(&mut self, other: &Square) {
      self.0 ^= *other as u16;
   }
}

impl Not for BitBoard {
   type Output = BitBoard;

   #[inline]
   fn not(self) -> BitBoard {
      BitBoard(!self.0)
   }
}

impl Not for &BitBoard {
   type Output = BitBoard;

   #[inline]
   fn not(self) -> BitBoard {
      BitBoard(!self.0)
   }
}
