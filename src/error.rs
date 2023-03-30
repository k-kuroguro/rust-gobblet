use std::{
   error,
   fmt::{Display, Formatter, Result},
};

use crate::Square;

/// Errors that may occur during working with a board.
#[derive(Clone, Copy, Debug)]
pub enum Error {
   /// An attempt was made to place a piece on an invalid square.
   /// e.g. A larger piece was already placed there.
   InvalidPlacing(Square),

   /// An attempt was made to move from or to an invalid square.
   /// e.g. `from` is a square where there is no piece.
   InvalidMoving { from: Square, to: Square },

   /// An attempt was made to take a piece from empty hand.
   EmptyHand,
}

impl Display for Error {
   fn fmt(&self, f: &mut Formatter) -> Result {
      use self::Error::*;

      match self {
         InvalidPlacing(square) => write!(f, "Couldn't place on {}.", square),
         InvalidMoving { from, to } => write!(f, "Couldn't move from {} to {}.", from, to),
         EmptyHand => write!(f, "Couldn't extract a piece from empty hand."),
      }
   }
}

impl error::Error for Error {}
