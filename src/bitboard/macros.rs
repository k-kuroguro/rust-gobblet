macro_rules! bit_and {
   ($left:ty, Square) => {
      impl ::std::ops::BitAnd<Square> for $left {
         type Output = BitBoard;

         #[inline]
         fn bitand(self, other: Square) -> BitBoard {
            BitBoard(self.0 & other as u16)
         }
      }
   };
   ($left:ty, &Square) => {
      impl ::std::ops::BitAnd<&Square> for $left {
         type Output = BitBoard;

         #[inline]
         fn bitand(self, other: &Square) -> BitBoard {
            BitBoard(self.0 & *other as u16)
         }
      }
   };
   ($left:ty, $right:ty) => {
      impl ::std::ops::BitAnd<$right> for $left {
         type Output = BitBoard;

         #[inline]
         fn bitand(self, other: $right) -> BitBoard {
            BitBoard(self.0 & other.0)
         }
      }
   };
}

macro_rules! bit_or {
   ($left:ty, Square) => {
      impl ::std::ops::BitOr<Square> for $left {
         type Output = BitBoard;

         #[inline]
         fn bitor(self, other: Square) -> BitBoard {
            BitBoard(self.0 | other as u16)
         }
      }
   };
   ($left:ty, &Square) => {
      impl ::std::ops::BitOr<&Square> for $left {
         type Output = BitBoard;

         #[inline]
         fn bitor(self, other: &Square) -> BitBoard {
            BitBoard(self.0 | *other as u16)
         }
      }
   };
   ($left:ty, $right:ty) => {
      impl ::std::ops::BitOr<$right> for $left {
         type Output = BitBoard;

         #[inline]
         fn bitor(self, other: $right) -> BitBoard {
            BitBoard(self.0 | other.0)
         }
      }
   };
}

macro_rules! bit_xor {
   ($left:ty, Square) => {
      impl ::std::ops::BitXor<Square> for $left {
         type Output = BitBoard;

         #[inline]
         fn bitxor(self, other: Square) -> BitBoard {
            BitBoard(self.0 ^ other as u16)
         }
      }
   };
   ($left:ty, &Square) => {
      impl ::std::ops::BitXor<&Square> for $left {
         type Output = BitBoard;

         #[inline]
         fn bitxor(self, other: &Square) -> BitBoard {
            BitBoard(self.0 ^ *other as u16)
         }
      }
   };
   ($left:ty, $right:ty) => {
      impl ::std::ops::BitXor<$right> for $left {
         type Output = BitBoard;

         #[inline]
         fn bitxor(self, other: $right) -> BitBoard {
            BitBoard(self.0 ^ other.0)
         }
      }
   };
}

macro_rules! bit_and_assign {
   ($left:ty, Square) => {
      impl ::std::ops::BitAndAssign<Square> for $left {
         #[inline]
         fn bitand_assign(&mut self, other: Square) {
            self.0 &= other as u16;
         }
      }
   };
   ($left:ty, &Square) => {
      impl ::std::ops::BitAndAssign<&Square> for $left {
         #[inline]
         fn bitand_assign(&mut self, other: &Square) {
            self.0 &= *other as u16;
         }
      }
   };
   ($left:ty, $right:ty) => {
      impl ::std::ops::BitAndAssign<$right> for $left {
         #[inline]
         fn bitand_assign(&mut self, other: $right) {
            self.0 &= other.0;
         }
      }
   };
}

macro_rules! bit_or_assign {
   ($left:ty, Square) => {
      impl ::std::ops::BitOrAssign<Square> for $left {
         #[inline]
         fn bitor_assign(&mut self, other: Square) {
            self.0 |= other as u16;
         }
      }
   };
   ($left:ty, &Square) => {
      impl ::std::ops::BitOrAssign<&Square> for $left {
         #[inline]
         fn bitor_assign(&mut self, other: &Square) {
            self.0 |= *other as u16;
         }
      }
   };
   ($left:ty, $right:ty) => {
      impl ::std::ops::BitOrAssign<$right> for $left {
         #[inline]
         fn bitor_assign(&mut self, other: $right) {
            self.0 |= other.0;
         }
      }
   };
}

macro_rules! bit_xor_assign {
   ($left:ty, Square) => {
      impl ::std::ops::BitXorAssign<Square> for $left {
         #[inline]
         fn bitxor_assign(&mut self, other: Square) {
            self.0 ^= other as u16;
         }
      }
   };
   ($left:ty, &Square) => {
      impl ::std::ops::BitXorAssign<&Square> for $left {
         #[inline]
         fn bitxor_assign(&mut self, other: &Square) {
            self.0 ^= *other as u16;
         }
      }
   };
   ($left:ty, $right:ty) => {
      impl ::std::ops::BitXorAssign<$right> for $left {
         #[inline]
         fn bitxor_assign(&mut self, other: $right) {
            self.0 ^= other.0;
         }
      }
   };
}

macro_rules! not {
   ($type:ty) => {
      impl ::std::ops::Not for $type {
         type Output = BitBoard;

         #[inline]
         fn not(self) -> BitBoard {
            BitBoard(!self.0)
         }
      }
   };
}
