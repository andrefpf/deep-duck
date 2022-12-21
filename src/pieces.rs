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
    pub fn from_fen(notation: char) -> Option<Self> {
        match notation {
            'r' => Some(Piece::Rook(Color::Black)),
            'n' => Some(Piece::Knight(Color::Black)),
            'b' => Some(Piece::Bishop(Color::Black)),
            'q' => Some(Piece::Queen(Color::Black)),
            'k' => Some(Piece::King(Color::Black)),
            'p' => Some(Piece::Pawn(Color::Black)),

            'R' => Some(Piece::Rook(Color::White)),
            'N' => Some(Piece::Knight(Color::White)),
            'B' => Some(Piece::Bishop(Color::White)),
            'Q' => Some(Piece::Queen(Color::White)),
            'K' => Some(Piece::King(Color::White)),
            'P' => Some(Piece::Pawn(Color::White)),

            _ => None
        }
    }

    pub fn utf8_repr(&self) -> char {
        match self {
            Piece::Rook(Color::Black) => '♖',
            Piece::Knight(Color::Black) => '♘',
            Piece::Bishop(Color::Black) => '♗',
            Piece::Queen(Color::Black) => '♕',
            Piece::King(Color::Black) => '♔',
            Piece::Pawn(Color::Black) => '♙',

            Piece::Rook(Color::White) => '♜',
            Piece::Knight(Color::White) => '♞',
            Piece::Bishop(Color::White) => '♝',
            Piece::Queen(Color::White) => '♛',
            Piece::King(Color::White) => '♚',
            Piece::Pawn(Color::White) => '♟',
        }
    }

    pub fn fen_repr(&self) -> char {
        match self {
            Piece::Rook(Color::Black) => 'r',
            Piece::Knight(Color::Black) => 'n',
            Piece::Bishop(Color::Black) => 'b',
            Piece::Queen(Color::Black) => 'q',
            Piece::King(Color::Black) => 'k',
            Piece::Pawn(Color::Black) => 'p',

            Piece::Rook(Color::White) => 'R',
            Piece::Knight(Color::White) => 'N',
            Piece::Bishop(Color::White) => 'B',
            Piece::Queen(Color::White) => 'Q',
            Piece::King(Color::White) => 'K',
            Piece::Pawn(Color::White) => 'P',
        }
    }
}
