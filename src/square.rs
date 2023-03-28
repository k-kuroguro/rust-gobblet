// +----+----+----+----+-> X
// | A1 | B1 | C1 | D1 |
// +----+----+----+----+
// | A2 | B2 | C2 | D2 |
// +----+----+----+----+
// | A3 | B3 | C3 | D3 |
// +----+----+----+----+
// | A4 | B4 | C4 | D4 |
// +----+----+----+----+
// V Y

use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Square {
   A1 = 1 << 15,
   B1 = 1 << 14,
   C1 = 1 << 13,
   D1 = 1 << 12,
   A2 = 1 << 11,
   B2 = 1 << 10,
   C2 = 1 << 9,
   D2 = 1 << 8,
   A3 = 1 << 7,
   B3 = 1 << 6,
   C3 = 1 << 5,
   D3 = 1 << 4,
   A4 = 1 << 3,
   B4 = 1 << 2,
   C4 = 1 << 1,
   D4 = 1 << 0,
}

impl Square {
   pub const ALL: [Self; 16] = [
      Self::A1,
      Self::B1,
      Self::C1,
      Self::D1,
      Self::A2,
      Self::B2,
      Self::C2,
      Self::D2,
      Self::A3,
      Self::B3,
      Self::C3,
      Self::D3,
      Self::A4,
      Self::B4,
      Self::C4,
      Self::D4,
   ];

   pub fn from_pos(x: usize, y: usize) -> Self {
      Self::ALL[x.min(3) + 4 * y.min(3)]
   }
}

impl Display for Square {
   fn fmt(&self, f: &mut Formatter) -> Result {
      match self {
         Self::A1 => write!(f, "A1"),
         Self::B1 => write!(f, "B1"),
         Self::C1 => write!(f, "C1"),
         Self::D1 => write!(f, "D1"),
         Self::A2 => write!(f, "A2"),
         Self::B2 => write!(f, "B2"),
         Self::C2 => write!(f, "C2"),
         Self::D2 => write!(f, "D2"),
         Self::A3 => write!(f, "A3"),
         Self::B3 => write!(f, "B3"),
         Self::C3 => write!(f, "C3"),
         Self::D3 => write!(f, "D3"),
         Self::A4 => write!(f, "A4"),
         Self::B4 => write!(f, "B4"),
         Self::C4 => write!(f, "C4"),
         Self::D4 => write!(f, "D4"),
      }
   }
}
