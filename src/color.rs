/// A representation of piece/player color.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Color {
   Black = 0,
   White = 1,
}

impl Color {
   pub const NUM: usize = 2;

   /// An array that includes all the colors in the order of Black, White.
   pub const ALL: [Self; Self::NUM] = [Self::Black, Self::White];

   /// Returns the opposite color.
   pub const fn reverse(&self) -> Self {
      match self {
         Self::Black => Self::White,
         Self::White => Self::Black,
      }
   }
}
