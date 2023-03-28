use crate::color::Color;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum PieceKind {
   Tiny = 0,
   Small = 1,
   Medium = 2,
   Big = 3,
}

impl PieceKind {
   pub const NUM: usize = 4;

   pub const ALL: [Self; Self::NUM] = [Self::Big, Self::Medium, Self::Small, Self::Tiny];
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Piece {
   pub color: Color,
   pub kind: PieceKind,
}

impl Piece {
   pub const fn new(color: Color, kind: PieceKind) -> Self {
      Self { color, kind }
   }
}

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
