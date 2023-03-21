use crate::color::Color;

pub const PIECE_KIND_NUM: usize = 4;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum PieceKind {
   Tiny = 0,
   Small = 1,
   Medium = 2,
   Big = 3,
}

pub const ALL_PIECE_KINDS: [PieceKind; PIECE_KIND_NUM] = [
   PieceKind::Big,
   PieceKind::Medium,
   PieceKind::Small,
   PieceKind::Tiny,
];

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Piece {
   pub color: Color,
   pub kind: PieceKind,
}

impl Piece {
   pub fn new(color: Color, kind: PieceKind) -> Self {
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
