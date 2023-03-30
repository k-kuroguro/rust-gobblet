#[macro_use]
mod macros;

use crate::Square;

/// A simple bitboard.
/// You can work with this using bitwise operators.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct BitBoard(u16);

impl BitBoard {
   pub const EMPTY: Self = Self(0);

   /// Masks for the rows, columns, and diagonals of the board.
   pub const LINE_MASKS: [Self; 10] = [
      // Cols
      Self(0x8888),
      Self(0x4444),
      Self(0x2222),
      Self(0x1111),
      // Rows
      Self(0xF000),
      Self(0x0F00),
      Self(0x00F0),
      Self(0x000F),
      // Diagonals
      Self(0x8421),
      Self(0x1248),
   ];

   pub fn new(n: u16) -> Self {
      Self(n)
   }

   pub fn popcount(&self) -> u8 {
      self.0.count_ones() as u8
   }

   /// Returns a bitboard with the bit corresponding to the given square.
   pub fn from_square(square: Square) -> Self {
      Self(square as u16)
   }
}

bit_and!(BitBoard, BitBoard);
bit_and!(BitBoard, &BitBoard);
bit_and!(&BitBoard, BitBoard);
bit_and!(&BitBoard, &BitBoard);
bit_and!(BitBoard, Square);
bit_and!(BitBoard, &Square);
bit_and!(&BitBoard, Square);
bit_and!(&BitBoard, &Square);

bit_or!(BitBoard, BitBoard);
bit_or!(BitBoard, &BitBoard);
bit_or!(&BitBoard, BitBoard);
bit_or!(&BitBoard, &BitBoard);
bit_or!(BitBoard, Square);
bit_or!(BitBoard, &Square);
bit_or!(&BitBoard, Square);
bit_or!(&BitBoard, &Square);

bit_xor!(BitBoard, BitBoard);
bit_xor!(BitBoard, &BitBoard);
bit_xor!(&BitBoard, BitBoard);
bit_xor!(&BitBoard, &BitBoard);
bit_xor!(BitBoard, Square);
bit_xor!(BitBoard, &Square);
bit_xor!(&BitBoard, Square);
bit_xor!(&BitBoard, &Square);

bit_and_assign!(BitBoard, BitBoard);
bit_and_assign!(BitBoard, &BitBoard);
bit_and_assign!(BitBoard, Square);
bit_and_assign!(BitBoard, &Square);

bit_or_assign!(BitBoard, BitBoard);
bit_or_assign!(BitBoard, &BitBoard);
bit_or_assign!(BitBoard, Square);
bit_or_assign!(BitBoard, &Square);

bit_xor_assign!(BitBoard, BitBoard);
bit_xor_assign!(BitBoard, &BitBoard);
bit_xor_assign!(BitBoard, Square);
bit_xor_assign!(BitBoard, &Square);

not!(BitBoard);
not!(&BitBoard);

#[cfg(test)]
mod test {
   use crate::square::Square;

   use super::BitBoard;

   #[test]
   fn test_bitand() {
      let left_bitboard = BitBoard(0b0001);
      let right_bitboard = BitBoard(0b0011);
      let square = Square::D4; // 1 << 0
      let expected = BitBoard(0b0001);

      assert_eq!(left_bitboard & right_bitboard, expected);
      assert_eq!(left_bitboard & &right_bitboard, expected);
      assert_eq!(&left_bitboard & right_bitboard, expected);
      assert_eq!(&left_bitboard & &right_bitboard, expected);
      assert_eq!(left_bitboard & square, expected);
      assert_eq!(left_bitboard & &square, expected);
      assert_eq!(&left_bitboard & square, expected);
      assert_eq!(&left_bitboard & &square, expected);
   }

   #[test]
   fn test_bitor() {
      let left_bitboard = BitBoard(0b0001);
      let right_bitboard = BitBoard(0b0011);
      let square = Square::C4; // 1 << 1
      let expected = BitBoard(0b0011);

      assert_eq!(left_bitboard | right_bitboard, expected);
      assert_eq!(left_bitboard | &right_bitboard, expected);
      assert_eq!(&left_bitboard | right_bitboard, expected);
      assert_eq!(&left_bitboard | &right_bitboard, expected);
      assert_eq!(left_bitboard | square, expected);
      assert_eq!(left_bitboard | &square, expected);
      assert_eq!(&left_bitboard | square, expected);
      assert_eq!(&left_bitboard | &square, expected);
   }

   #[test]
   fn test_bitxor() {
      let left_bitboard = BitBoard(0b0011);
      let right_bitboard = BitBoard(0b0001);
      let square = Square::D4; // 1 << 0
      let expected = BitBoard(0b0010);

      assert_eq!(left_bitboard ^ right_bitboard, expected);
      assert_eq!(left_bitboard ^ &right_bitboard, expected);
      assert_eq!(&left_bitboard ^ right_bitboard, expected);
      assert_eq!(&left_bitboard ^ &right_bitboard, expected);
      assert_eq!(left_bitboard ^ square, expected);
      assert_eq!(left_bitboard ^ &square, expected);
      assert_eq!(&left_bitboard ^ square, expected);
      assert_eq!(&left_bitboard ^ &square, expected);
   }

   #[test]
   fn test_not() {
      let bitboard = BitBoard(0xF0F0);
      let expected = BitBoard(0x0F0F);

      assert_eq!(!bitboard, expected);
      assert_eq!(!&bitboard, expected);
   }
}
