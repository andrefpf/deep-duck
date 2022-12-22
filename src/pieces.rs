#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
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

#[derive(Copy, Clone, Debug)]
pub struct Position(pub i32, pub i32);

impl Piece {
    pub fn from_fen(notation: char) -> Self {
        match notation {
            'r' => Piece{color:Color::Black, kind:PieceKind::Rook},
            'n' => Piece{color:Color::Black, kind:PieceKind::Knight},
            'b' => Piece{color:Color::Black, kind:PieceKind::Bishop},
            'q' => Piece{color:Color::Black, kind:PieceKind::Queen},
            'k' => Piece{color:Color::Black, kind:PieceKind::King},
            'p' => Piece{color:Color::Black, kind:PieceKind::Pawn},

            'R' => Piece{color:Color::White, kind:PieceKind::Rook},
            'N' => Piece{color:Color::White, kind:PieceKind::Knight},
            'B' => Piece{color:Color::White, kind:PieceKind::Bishop},
            'Q' => Piece{color:Color::White, kind:PieceKind::Queen},
            'K' => Piece{color:Color::White, kind:PieceKind::King},
            'P' => Piece{color:Color::White, kind:PieceKind::Pawn},

            _ => panic!("Unkown fen piece"),
        }
    }

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

    pub fn fen_repr(&self) -> char {
        match (self.color, self.kind) {
            (Color::Black, PieceKind::Rook) => 'r',
            (Color::Black, PieceKind::Knight) => 'n',
            (Color::Black, PieceKind::Bishop) => 'b',
            (Color::Black, PieceKind::Queen) => 'q',
            (Color::Black, PieceKind::King) => 'k',
            (Color::Black, PieceKind::Pawn) => 'p',

            (Color::White, PieceKind::Rook) => 'R',
            (Color::White, PieceKind::Knight) => 'N',
            (Color::White, PieceKind::Bishop) => 'B',
            (Color::White, PieceKind::Queen) => 'Q',
            (Color::White, PieceKind::King) => 'K',
            (Color::White, PieceKind::Pawn) => 'P',
        }
    }
}
