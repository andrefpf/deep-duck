#[derive(Copy, Clone, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug)]
pub enum Piece {
    Pawn(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Queen(Color),
    King(Color),
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Piece {
    pub fn repr(piece: Option<Self>) -> char {
        match piece {
            Some(Piece::Rook(Color::Black)) => '♖',
            Some(Piece::Knight(Color::Black)) => '♘',
            Some(Piece::Bishop(Color::Black)) => '♗',
            Some(Piece::Queen(Color::Black)) => '♕',
            Some(Piece::King(Color::Black)) => '♔',
            Some(Piece::Pawn(Color::Black)) => '♙',

            Some(Piece::Rook(Color::White)) => '♜',
            Some(Piece::Knight(Color::White)) => '♞',
            Some(Piece::Bishop(Color::White)) => '♝',
            Some(Piece::Queen(Color::White)) => '♛',
            Some(Piece::King(Color::White)) => '♚',
            Some(Piece::Pawn(Color::White)) => '♟',

            None => ' ',
        }
    }
}
