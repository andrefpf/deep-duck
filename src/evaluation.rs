use crate::board::Board;
use crate::pieces::PieceKind;
use crate::pieces::Piece;
use crate::pieces::Color;


const PAWN_TABLE: [i32; 64] = [
 0,  0,  0,  0,  0,  0,  0,  0,
50, 50, 50, 50, 50, 50, 50, 50,
10, 10, 20, 30, 30, 20, 10, 10,
 5,  5, 10, 25, 25, 10,  5,  5,
 0,  0,  0, 20, 20,  0,  0,  0,
 5, -5,-10,  0,  0,-10, -5,  5,
 5, 10, 10,-20,-20, 10, 10,  5,
 0,  0,  0,  0,  0,  0,  0,  0,
];

const KNIGHT_TABLE: [i32; 64] = [
-50,-40,-30,-30,-30,-30,-40,-50,
-40,-20,  0,  0,  0,  0,-20,-40,
-30,  0, 10, 15, 15, 10,  0,-30,
-30,  5, 15, 20, 20, 15,  5,-30,
-30,  0, 15, 20, 20, 15,  0,-30,
-30,  5, 10, 15, 15, 10,  5,-30,
-40,-20,  0,  5,  5,  0,-20,-40,
-50,-40,-30,-30,-30,-30,-40,-50,
];

const BISHOP_TABLE: [i32; 64] = [
-20,-10,-10,-10,-10,-10,-10,-20,
-10,  0,  0,  0,  0,  0,  0,-10,
-10,  0,  5, 10, 10,  5,  0,-10,
-10,  5,  5, 10, 10,  5,  5,-10,
-10,  0, 10, 10, 10, 10,  0,-10,
-10, 10, 10, 10, 10, 10, 10,-10,
-10,  5,  0,  0,  0,  0,  5,-10,
-20,-10,-10,-10,-10,-10,-10,-20,
];

const ROOK_TABLE: [i32; 64] = [
 0,  0,  0,  0,  0,  0,  0,  0,
 5, 10, 10, 10, 10, 10, 10,  5,
-5,  0,  0,  0,  0,  0,  0, -5,
-5,  0,  0,  0,  0,  0,  0, -5,
-5,  0,  0,  0,  0,  0,  0, -5,
-5,  0,  0,  0,  0,  0,  0, -5,
-5,  0,  0,  0,  0,  0,  0, -5,
 0,  0,  0,  5,  5,  0,  0,  0
];

const QUEEN_TABLE: [i32; 64] = [
-20,-10,-10, -5, -5,-10,-10,-20,
-10,  0,  0,  0,  0,  0,  0,-10,
-10,  0,  5,  5,  5,  5,  0,-10,
 -5,  0,  5,  5,  5,  5,  0, -5,
  0,  0,  5,  5,  5,  5,  0, -5,
-10,  5,  5,  5,  5,  5,  0,-10,
-10,  0,  5,  0,  0,  0,  0,-10,
-20,-10,-10, -5, -5,-10,-10,-20
];

const KING_TABLE: [i32; 64] = [
-30,-40,-40,-50,-50,-40,-40,-30,
-30,-40,-40,-50,-50,-40,-40,-30,
-30,-40,-40,-50,-50,-40,-40,-30,
-30,-40,-40,-50,-50,-40,-40,-30,
-20,-30,-30,-40,-40,-30,-30,-20,
-10,-20,-20,-20,-20,-20,-20,-10,
 20, 20,  0,  0,  0,  0, 20, 20,
 20, 30, 10,  0,  0, 10, 30, 20
];

pub fn count_centipawns(board: &Board) -> i32 {
    let mut score: i32 = 0;
    
    for piece in board.ocuppied_squares() {      
        if piece.color == board.active_color {
            score += square_value(piece);
        } else {
            score -= square_value(piece);
        }
    }

    score
}

pub fn square_value(piece: Piece) -> i32 {
    position_value(piece) + piece_value(piece.kind)
}

pub fn position_value(piece: Piece) -> i32 {
    let table = piece_table(piece.kind);
    let mut index = piece.pos.0 + 8 * piece.pos.1;

    if let Color::Black = piece.color {
        index = 63 - index;
    }

    table[index as usize]
}

pub fn piece_value(piece_kind: PieceKind) -> i32 {
    match piece_kind {
        PieceKind::Pawn => 100,
        PieceKind::Bishop => 350,
        PieceKind::Knight => 650,
        PieceKind::Rook => 550,
        PieceKind::Queen => 1000,
        PieceKind::King => 1_000_000,
        PieceKind::Duck => 0,
    }
}

fn piece_table(piece_kind: PieceKind) -> [i32; 64] {
    match piece_kind {
        PieceKind::Pawn => PAWN_TABLE,
        PieceKind::Bishop => BISHOP_TABLE,
        PieceKind::Knight => KNIGHT_TABLE,
        PieceKind::Rook => ROOK_TABLE,
        PieceKind::Queen => QUEEN_TABLE,
        PieceKind::King => KING_TABLE,
        PieceKind::Duck => KNIGHT_TABLE, // seems ok
    }
}