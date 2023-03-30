use crate::Color;

/// Represents the size of piece.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Size {
   Tiny = 0,
   Small = 1,
   Medium = 2,
   Big = 3,
}

impl Size {
   pub const NUM: usize = 4;

   /// An array that includes all the sizes in the order of Tiny, Small, Medium, Big.
   pub const ALL: [Self; Self::NUM] = [Self::Tiny, Self::Small, Self::Medium, Self::Big];
}

/// Represents the piece that includes color and size.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Piece {
   pub color: Color,
   pub size: Size,
}

impl Piece {
   pub const fn new(color: Color, size: Size) -> Self {
      Self { color, size }
   }
}

/// Represents the set of piece.
/// This is used like a stack.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct PieceSet(Vec<Piece>);

impl PieceSet {
   pub fn from_vec(vec: Vec<Piece>) -> Self {
      Self(vec)
   }

   pub fn from_slice(slice: &[Piece]) -> Self {
      Self(slice.to_vec())
   }

   pub fn none() -> Self {
      Self(Vec::new())
   }

   pub fn peek(&self) -> Option<&Piece> {
      self.0.last()
   }

   pub fn pop(&mut self) -> Option<Piece> {
      self.0.pop()
   }
}
