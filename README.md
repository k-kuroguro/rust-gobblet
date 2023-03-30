# Rust Gobblet Library

This is a library for management Gobblet game.

## Example

### Basic Game Progress

```rust
use gobblet::{Action, Color, Game, Piece, Size, Square, Status};

let mut game = Game::new();

// Place pieces from hand.
assert_eq!(game.turn(), Color::Black);
game.execute(Action::PlaceFromHand {index: 0, to: Square::B2});
assert_eq!(game.turn(), Color::White);
game.execute(Action::PlaceFromHand {index: 0, to: Square::C3});

assert_eq!(game.board().get_top(Square::B2), Some(Piece::new(Color::Black, Size::Big)));
assert_eq!(game.board().get_top(Square::C3), Some(Piece::new(Color::White, Size::Big)));


// Move piece.
assert_eq!(game.turn(), Color::Black);
game.execute(Action::Move {from: Square::B2, to: Square::A2});

assert_eq!(game.board().get_top(Square::B2), None);
assert_eq!(game.board().get_top(Square::A2), Some(Piece::new(Color::Black, Size::Big)));


// Judge the winner.
game.execute(Action::PlaceFromHand {index: 0, to: Square::C2});
game.execute(Action::PlaceFromHand {index: 0, to: Square::A1});
game.execute(Action::PlaceFromHand {index: 0, to: Square::C1});

let status = game.execute(Action::PlaceFromHand {index: 0, to: Square::B1}).unwrap();
assert_eq!(status, Status::OnGoing);

let status = game.execute(Action::PlaceFromHand {index: 0, to: Square::C4}).unwrap();
assert_eq!(status, Status::WhiteWins);
```

### Get Pieces On Board

```rust
use gobblet::{Action, Color, Game, Piece, PieceSet, Size, Square};

let mut game = Game::new();

game.execute(Action::PlaceFromHand {index: 0, to: Square::B2});
game.execute(Action::PlaceFromHand {index: 0, to: Square::C3});
game.execute(Action::PlaceFromHand {index: 0, to: Square::A4});
game.execute(Action::PlaceFromHand {index: 0, to: Square::D1});
game.execute(Action::Move {from: Square::B2, to: Square::A4});

let D1 = PieceSet::from_slice(&[Piece::new(Color::White, Size::Medium)]);
let C3 = PieceSet::from_slice(&[Piece::new(Color::White, Size::Big)]);
let A4 = PieceSet::from_slice(&[Piece::new(Color::Black, Size::Medium), Piece::new(Color::Black, Size::Big)]);
for (i, set) in game.board().into_iter().enumerate() {
   match i {
      3 => assert_eq!(set, D1),
      10 => assert_eq!(set, C3),
      12 => assert_eq!(set, A4),
      _ => assert_eq!(set, PieceSet::none()),
   }
}
```

## Documentation

See https://k-kuroguro.github.io/rust-gobblet/gobblet/.
