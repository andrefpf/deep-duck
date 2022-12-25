use std::cmp;
use crate::board::Board;
use crate::pieces::PieceKind;
use crate::pieces::Position;
use crate::pieces::Piece;
use crate::pieces::Color;
use crate::movements::Movement;

pub fn search(board: &Board, color: Color) -> Option<Movement> {
    let iterations = 3;

    let mut best_movement: Option<Movement> = None;
    let mut best_score: i32 = i32::MIN;

    let mut tmp_score;
    let mut tmp_board: Board;

    let next_color = match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    let avaliable_moves = Movement::avaliable_moves(board, color);

    for movement in avaliable_moves {
        tmp_board = board.copy_and_move(movement.origin, movement.target);
        tmp_score = -evaluate_recursive(&tmp_board, next_color, iterations);
        if best_score < tmp_score {
            best_score = tmp_score;
            best_movement = Some(movement);
        }
    }

    best_movement
}

pub fn evaluate(board: &Board, color: Color) -> i32 {
    count_material(board, color)
}

fn evaluate_recursive(board: &Board, color: Color, iterations: usize) -> i32 {
    if iterations == 0 {
        return evaluate(board, color);
    }
    
    let mut tmp_board: Board;
    let mut current_score;
    let mut best_score = i32::MIN;
    
    let next_color = match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    for movement in Movement::avaliable_moves(board, color) {
        tmp_board = board.copy_and_move(movement.origin, movement.target);
        current_score = -evaluate_recursive(&tmp_board, next_color, iterations-1);
        best_score = cmp::max(best_score, current_score);
    }

    best_score
}

fn count_material(board: &Board, color: Color) -> i32 {
    let mut score: i32 = 0;
    
    for i in 0..8 {
        for j in 0..8 {
            let position = Position(i, j);
            let piece = board.get_piece(position);
            
            if let None = piece {
                continue;
            }
            
            let piece = piece.unwrap();
            
            if piece.color == color {
                score = score + piece_value(piece);
            } else {
                score = score - piece_value(piece);
            }     
        }
    }

    score
}

fn piece_value(piece: Piece) -> i32 {
    match piece.kind {
        PieceKind::Pawn => 100,
        PieceKind::Bishop => 300,
        PieceKind::Knight => 300,
        PieceKind::Rook => 500,
        PieceKind::Queen => 900,
        PieceKind::King => 1_000_000,
    }
}