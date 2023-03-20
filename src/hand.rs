use crate::piece::Piece;

const PIECE_SET_NUM: usize = 3;
const INITIAL_GROUP: [Piece; 4] = [Piece::Tiny, Piece::Small, Piece::Medium, Piece::Big];

#[derive(Clone, Debug)]
struct PieceSet(Vec<Piece>);

impl PieceSet {
   pub fn new() -> Self {
      Self(INITIAL_GROUP.to_vec())
   }

   pub fn peek(&self) -> Option<&Piece> {
      self.0.last()
   }

   pub fn pop(&mut self) -> Option<Piece> {
      self.0.pop()
   }
}

#[derive(Clone, Debug)]
pub struct Hand {
   pieces: [PieceSet; PIECE_SET_NUM],
}

impl Hand {
   pub fn new() -> Self {
      Self {
         pieces: [PieceSet::new(), PieceSet::new(), PieceSet::new()],
      }
   }

   pub fn peek(&self, i: usize) -> Option<&Piece> {
      self.pieces[i].peek()
   }

   pub fn pop(&mut self, i: usize) -> Option<Piece> {
      self.pieces[i].pop()
   }
}
