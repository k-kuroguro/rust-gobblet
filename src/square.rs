// +----+----+----+----+
// | A1 | B1 | C1 | D1 |
// +----+----+----+----+
// | A2 | B2 | C2 | D2 |
// +----+----+----+----+
// | A3 | B3 | C3 | D3 |
// +----+----+----+----+
// | A4 | B4 | C4 | D4 |
// +----+----+----+----+

use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, Debug)]
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

pub const ALL_SQUARES: [Square; 16] = [
   Square::A1,
   Square::B1,
   Square::C1,
   Square::D1,
   Square::A2,
   Square::B2,
   Square::C2,
   Square::D2,
   Square::A3,
   Square::B3,
   Square::C3,
   Square::D3,
   Square::A4,
   Square::B4,
   Square::C4,
   Square::D4,
];

impl Display for Square {
   fn fmt(&self, f: &mut Formatter) -> Result {
      match self {
         Square::A1 => write!(f, "A1"),
         Square::B1 => write!(f, "B1"),
         Square::C1 => write!(f, "C1"),
         Square::D1 => write!(f, "D1"),
         Square::A2 => write!(f, "A2"),
         Square::B2 => write!(f, "B2"),
         Square::C2 => write!(f, "C2"),
         Square::D2 => write!(f, "D2"),
         Square::A3 => write!(f, "A3"),
         Square::B3 => write!(f, "B3"),
         Square::C3 => write!(f, "C3"),
         Square::D3 => write!(f, "D3"),
         Square::A4 => write!(f, "A4"),
         Square::B4 => write!(f, "B4"),
         Square::C4 => write!(f, "C4"),
         Square::D4 => write!(f, "D4"),
      }
   }
}
