use crate::board::Board;
use crate::pieces::Piece;
use crate::pieces::PieceKind;
use crate::pieces::Color;
use crate::pieces::Position;

#[derive(Copy, Clone, Debug)]
pub struct Movement {
    origin: Position,
    target: Position,
}

pub fn piece_moves(board: &Board, origin: Position) -> Vec::<Movement> {
    let piece = match board.get_piece(origin) {
        Some(p) => p,
        None => return Vec::<Movement>::new(), // goes out if None
    };

    match piece.kind {
        PieceKind::Rook => rook_moves(board, origin),
        _ => Vec::<Movement>::new(),
    }
}

fn rook_moves(board: &Board, origin: Position) -> Vec::<Movement> {
    let mut movements = Vec::<Movement>::new();

    let piece = match board.get_piece(origin) {
        Some(p) => p,
        None => return Vec::<Movement>::new(), // goes out if None
    };

    let start = origin.0 + 1;
    let end = 8;
    for i in start..end {
        let target = Position(i, origin.1);

        if let Some(target_piece) = board.get_piece(target) {
            if piece.color != target_piece.color {
                movements.push(Movement{origin, target});
            }
            break;
        } else {
            movements.push(Movement{origin, target});
        }
    }

    let start = 0;
    let end = origin.0;
    for i in (start..end).rev() {
        let target = Position(i, origin.1);

        if let Some(target_piece) = board.get_piece(target) {
            if piece.color != target_piece.color {
                movements.push(Movement{origin, target});
            }
            break;
        } else {
            movements.push(Movement{origin, target});
        }
    }

    let start = origin.1 + 1;
    let end = 8;
    for i in start..end {
        let target = Position(origin.0, i);

        if let Some(target_piece) = board.get_piece(target) {
            if piece.color != target_piece.color {
                movements.push(Movement{origin, target});
            }
            break;
        } else {
            movements.push(Movement{origin, target});
        }
    }

    let start = 0;
    let end = origin.1;
    for i in (start..end).rev() {
        let target = Position(origin.0, i);

        if let Some(target_piece) = board.get_piece(target) {
            if piece.color != target_piece.color {
                movements.push(Movement{origin, target});
            }
            break;
        } else {
            movements.push(Movement{origin, target});
        }
    }

    movements
}