use std::cmp;
use crate::board::Board;
use crate::pieces::PieceKind;
use crate::pieces::Piece;
use crate::pieces::Color;
use crate::movements::Movement;

pub fn search(board: &Board, color: Color, depth: usize) -> Option<Movement> {
    let mut best_movement: Option<Movement> = None;
    let mut best_score = i32::MIN;

    let mut tmp_score: i32;
    let mut tmp_board: Board;

    let next_color = match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    let avaliable_moves = Movement::avaliable_moves(board, color);

    for movement in avaliable_moves {
        tmp_board = board.copy_and_move(movement.origin, movement.target);
        tmp_score = -evaluate_recursive(&tmp_board, next_color, depth, -i32::MAX, i32::MAX);
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

fn evaluate_recursive(board: &Board, color: Color, depth: usize, alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        return evaluate(board, color);
    }
    
    let mut alpha = alpha;
    let mut tmp_score: i32;
    let mut tmp_board: Board;

    let next_color = match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    let avaliable_moves = Movement::avaliable_moves(board, color);

    for mv in avaliable_moves {
        tmp_board = board.copy_and_move(mv.origin, mv.target);
        tmp_score = -evaluate_recursive(&tmp_board, next_color, depth - 1, -beta, -alpha);
        
        if tmp_score >= beta {
            return beta;
        }
        alpha = cmp::max(alpha, tmp_score);
    }

    alpha
}

fn count_material(board: &Board, color: Color) -> i32 {
    let mut score: i32 = 0;
    
    for square in board.ocuppied_squares() {
        let piece = square.piece.unwrap();
            
        if piece.color == color {
            score = score + piece_value(piece);
        } else {
            score = score - piece_value(piece);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obvious() {
        let mut board = Board::from_fen("4k3/8/5r2/2KN4/8/8/8/8 w - - 0 1");
        let best_move = search(&board, Color::White, 2);
        
        if let None = best_move {
            panic!("No moves found");
        }
        let best_move = best_move.unwrap();
        
        assert_eq!(best_move.origin, Position(3, 4));
        assert_eq!(best_move.target, Position(5, 5));
    }
    
    #[test]
    fn test_forks() {
        let mut board = Board::from_fen("r3k3/2b5/5r2/2KN4/8/8/8/8 w - - 0 1");
        let best_move = search(&board, Color::White, 2);

        if let None = best_move {
            panic!("No moves found");
        }
        let best_move = best_move.unwrap();

        assert_eq!(best_move.origin, Position(3, 4));
        assert_eq!(best_move.target, Position(2, 6));
    }
}