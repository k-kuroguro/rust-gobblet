use std::array;

use crate::{Color, Piece, PieceSet, Size};

const INITIAL_SETS: [[Piece; 4]; 2] = [
   [
      Piece {
         color: Color::Black,
         size: Size::Tiny,
      },
      Piece {
         color: Color::Black,
         size: Size::Small,
      },
      Piece {
         color: Color::Black,
         size: Size::Medium,
      },
      Piece {
         color: Color::Black,
         size: Size::Big,
      },
   ],
   [
      Piece {
         color: Color::White,
         size: Size::Tiny,
      },
      Piece {
         color: Color::White,
         size: Size::Small,
      },
      Piece {
         color: Color::White,
         size: Size::Medium,
      },
      Piece {
         color: Color::White,
         size: Size::Big,
      },
   ],
];

/// Represents player's hand.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Hand {
   sets: [PieceSet; Self::PIECE_SET_NUM],
}

impl Hand {
   /// The number of piece sets in player's hand.
   pub const PIECE_SET_NUM: usize = 3;

   pub fn new(color: Color) -> Self {
      Self {
         sets: [
            PieceSet::from_slice(&INITIAL_SETS[color as usize]),
            PieceSet::from_slice(&INITIAL_SETS[color as usize]),
            PieceSet::from_slice(&INITIAL_SETS[color as usize]),
         ],
      }
   }

   /// Fetch the top piece without removing it.
   pub fn peek(&self, i: usize) -> Option<&Piece> {
      self.sets[i.clamp(0, Self::PIECE_SET_NUM - 1)].peek()
   }

   /// Returns and removes the top piece.
   pub fn pop(&mut self, i: usize) -> Option<Piece> {
      self.sets[i.clamp(0, Self::PIECE_SET_NUM - 1)].pop()
   }
}

impl IntoIterator for Hand {
   type Item = PieceSet;
   type IntoIter = array::IntoIter<PieceSet, { Hand::PIECE_SET_NUM }>;

   fn into_iter(self) -> Self::IntoIter {
      self.sets.into_iter()
   }
}

impl IntoIterator for &Hand {
   type Item = PieceSet;
   type IntoIter = array::IntoIter<PieceSet, { Hand::PIECE_SET_NUM }>;

   fn into_iter(self) -> Self::IntoIter {
      self.sets.clone().into_iter()
   }
}
