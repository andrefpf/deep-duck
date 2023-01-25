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

#[derive(Copy, Clone, Debug, PartialEq)]
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

impl Color {
    pub fn invert(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
            Color::Yellow => Color::Yellow,
        }
    }
}

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

            (Color::Yellow, PieceKind::Duck) => '🐤',
            _ => panic!("Invalid piece."),
        }
    }
}

#[allow(dead_code)]
impl Position {
    pub fn from_str(input: String) -> Option<Self> {
        let mut chars = input.chars();
        let file = chars.next();
        let rank = chars.next();

        let x = match rank {
            Some('A') => 0,
            Some('B') => 1,
            Some('C') => 2,
            Some('D') => 3,
            Some('E') => 4,
            Some('F') => 5,
            Some('G') => 6,
            Some('H') => 7,
            _ => return None,
        };

        let y = match file {
            Some('0') => 0,
            Some('1') => 1,
            Some('2') => 2,
            Some('3') => 3,
            Some('4') => 4,
            Some('5') => 5,
            Some('6') => 6,
            Some('7') => 7,
            _ => return None,
        };

        Some(Position(x, y))
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