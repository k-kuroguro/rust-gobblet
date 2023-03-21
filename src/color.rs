pub const COLOR_NUM: usize = 2;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Color {
   Black = 0,
   White = 1,
}

impl Color {
   pub fn reverse(&self) -> Self {
      match self {
         Color::Black => Color::White,
         Color::White => Color::Black,
      }
   }
}

pub const ALL_COLORS: [Color; COLOR_NUM] = [Color::Black, Color::White];
