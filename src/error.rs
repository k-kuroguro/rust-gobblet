use std::{
   error,
   fmt::{Display, Formatter, Result},
};

use crate::square::Square;

#[derive(Clone, Copy, Debug)]
pub enum Error {
   InvalidPlacing(Square),
   InvalidMoving { from: Square, to: Square },
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
