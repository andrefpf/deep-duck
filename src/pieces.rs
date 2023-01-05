#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub pos: Position,
    pub color: Color,
    pub kind: PieceKind,
}

#[derive(Copy, Clone, Debug)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position(pub i32, pub i32);

impl Piece {
    pub fn utf8_repr(&self) -> char {
        match (self.color, self.kind) {
            (Color::Black, PieceKind::Rook) => '♖',
            (Color::Black, PieceKind::Knight) => '♘',
            (Color::Black, PieceKind::Bishop) => '♗',
            (Color::Black, PieceKind::Queen) => '♕',
            (Color::Black, PieceKind::King) => '♔',
            (Color::Black, PieceKind::Pawn) => '♙',

            (Color::White, PieceKind::Rook) => '♜',
            (Color::White, PieceKind::Knight) => '♞',
            (Color::White, PieceKind::Bishop) => '♝',
            (Color::White, PieceKind::Queen) => '♛',
            (Color::White, PieceKind::King) => '♚',
            (Color::White, PieceKind::Pawn) => '♟',
        }
    }
}
