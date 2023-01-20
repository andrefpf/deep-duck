use std::fmt;


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
    Yellow,
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
    Duck,
}

#[derive(Copy, Clone, PartialEq)]
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

            (Color::Yellow, PieceKind::Duck) => '☻',
            _ => panic!("Invalid piece."),
        }
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let list = "ABCDEFGH";
        let possible_index = list.chars().nth(self.0 as usize);
        
        if let Some(index) = possible_index {
            let data = format!("{}{}", index, self.1 + 1);
            write!(f, "{}", data)
        } else {
            panic!("Invalid position");
        }
    }
}