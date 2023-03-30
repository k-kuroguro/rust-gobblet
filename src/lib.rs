//! # Rust Gobblet Library
//! This is a library for management Gobblet game.
//!
//! ## Example
//! This places black piece on B2 and white piece on C3 from hands.
//!
//! ```
//! use gobblet::{Action, Color, Game, Piece, Size, Square};
//!
//! let mut game = Game::new();
//!
//! assert_eq!(game.turn(), Color::Black);
//! game.execute(Action::PlaceFromHand {index: 0, to: Square::B2});
//! assert_eq!(game.turn(), Color::White);
//! game.execute(Action::PlaceFromHand {index: 0, to: Square::C3});
//!
//! assert_eq!(game.board().get_top(Square::B2), Some(Piece::new(Color::Black, Size::Big)));
//! assert_eq!(game.board().get_top(Square::C3), Some(Piece::new(Color::White, Size::Big)));
//! ```

mod bitboard;
pub use crate::bitboard::*;

mod board;
pub use crate::board::*;

mod color;
pub use crate::color::*;

mod error;
pub use crate::error::*;

mod game;
pub use crate::game::*;

mod hand;
pub use crate::hand::*;

mod piece;
pub use crate::piece::*;

mod square;
pub use crate::square::*;
