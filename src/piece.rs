pub const PIECE_NUM: usize = 4;

#[derive(Clone, Copy, Debug)]
pub enum Piece {
   Tiny = 0,
   Small = 1,
   Medium = 2,
   Big = 3,
}

pub const ALL_PIECES: [Piece; PIECE_NUM] = [Piece::Big, Piece::Medium, Piece::Small, Piece::Tiny];
