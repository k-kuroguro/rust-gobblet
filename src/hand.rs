use std::array;

use crate::{
   color::Color,
   piece::{Piece, PieceKind, PieceSet},
};

pub const PIECE_SET_NUM: usize = 3;
const INITIAL_SETS: [[Piece; 4]; 2] = [
   [
      Piece {
         color: Color::Black,
         kind: PieceKind::Tiny,
      },
      Piece {
         color: Color::Black,
         kind: PieceKind::Small,
      },
      Piece {
         color: Color::Black,
         kind: PieceKind::Medium,
      },
      Piece {
         color: Color::Black,
         kind: PieceKind::Big,
      },
   ],
   [
      Piece {
         color: Color::White,
         kind: PieceKind::Tiny,
      },
      Piece {
         color: Color::White,
         kind: PieceKind::Small,
      },
      Piece {
         color: Color::White,
         kind: PieceKind::Medium,
      },
      Piece {
         color: Color::White,
         kind: PieceKind::Big,
      },
   ],
];

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Hand {
   sets: [PieceSet; PIECE_SET_NUM],
}

impl Hand {
   pub fn new(color: Color) -> Self {
      Self {
         sets: [
            PieceSet::from_slice(&INITIAL_SETS[color as usize]),
            PieceSet::from_slice(&INITIAL_SETS[color as usize]),
            PieceSet::from_slice(&INITIAL_SETS[color as usize]),
         ],
      }
   }

   pub fn peek(&self, i: usize) -> Option<&Piece> {
      self.sets[i.clamp(0, PIECE_SET_NUM - 1)].peek()
   }

   pub fn pop(&mut self, i: usize) -> Option<Piece> {
      self.sets[i.clamp(0, PIECE_SET_NUM - 1)].pop()
   }
}

impl IntoIterator for Hand {
   type Item = PieceSet;
   type IntoIter = array::IntoIter<PieceSet, PIECE_SET_NUM>;

   fn into_iter(self) -> Self::IntoIter {
      self.sets.into_iter()
   }
}

impl IntoIterator for &Hand {
   type Item = PieceSet;
   type IntoIter = array::IntoIter<PieceSet, PIECE_SET_NUM>;

   fn into_iter(self) -> Self::IntoIter {
      self.sets.clone().into_iter()
   }
}
