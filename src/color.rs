#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Color {
   Black = 0,
   White = 1,
}

impl Color {
   pub const NUM: usize = 2;

   pub const ALL: [Self; Self::NUM] = [Self::Black, Self::White];

   pub const fn reverse(&self) -> Self {
      match self {
         Self::Black => Self::White,
         Self::White => Self::Black,
      }
   }
}
