use std::cmp;
use crate::board::Board;
use crate::pieces::PieceKind;
use crate::pieces::Piece;
use crate::movements::Movement;

#[derive(Copy, Clone, Debug)]
struct Prune {
    alpha: i32,
    beta: i32,
}

pub fn search(board: &Board, depth: usize) -> Option<Movement> {
    let mut best_movement: Option<Movement> = None;
    let mut avaliable_moves = Movement::avaliable_moves(board, false);
    avaliable_moves.sort_by_cached_key(|x| -estimate_movement(&x));

    let mut prune = Prune {
        alpha: -i32::MAX,
        beta: i32::MAX,
    };
    
    for movement in avaliable_moves {
        let tmp_prune = Prune{
            alpha: -prune.beta, 
            beta: -prune.alpha, 
        };
        let tmp_board = board.copy_movement(movement);
        let tmp_score = -evaluate_recursive(&tmp_board, depth-1, tmp_prune);

        if tmp_score >= prune.beta {
            return best_movement;
        }

        if tmp_score > prune.alpha {
            prune.alpha = tmp_score;
            best_movement = Some(movement);
        }
    }

    best_movement
}

pub fn evaluate(board: &Board, depth: usize) -> i32 {
    let mut prune = Prune {
        alpha: -i32::MAX,
        beta: i32::MAX,
    };
    evaluate_recursive(board, depth, prune)
}

fn evaluate_static(board: &Board) -> i32 {
    count_material(board)
}

fn evaluate_recursive(board: &Board, depth: usize, prune: Prune) -> i32 {
    if depth == 0 {
        return evaluate_critic(board, 5, prune);
    }
    
    let mut avaliable_moves = Movement::avaliable_moves(board, false);
    avaliable_moves.sort_by_cached_key(|x| -estimate_movement(&x));
    
    if avaliable_moves.len() == 0 {
        return evaluate_critic(board, 5, prune);
    }
    
    let mut prune = prune;

    for movement in avaliable_moves {
        let tmp_prune = Prune{
            alpha: -prune.beta, 
            beta: -prune.alpha, 
        };
        let tmp_board = board.copy_movement(movement);
        let tmp_score = -evaluate_recursive(&tmp_board, depth - 1, tmp_prune);
        
        if tmp_score >= prune.beta {
            return prune.beta;
        }

        if tmp_score > prune.alpha {
            prune.alpha = tmp_score;
        }
    }

    prune.alpha
}

fn evaluate_critic(board: &Board, depth: usize, prune: Prune) -> i32 {
    if depth == 0 {
        return evaluate_static(board);
    }
    
    let mut avaliable_moves = Movement::avaliable_moves(board, true);
    avaliable_moves.sort_by_cached_key(|x| -estimate_movement(&x));
    
    if avaliable_moves.len() == 0 {
        return evaluate_static(board);
    }
    
    let mut prune = prune;

    for movement in avaliable_moves {
        let tmp_prune = Prune{
            alpha: -prune.beta, 
            beta: -prune.alpha, 
        };
        let tmp_board = board.copy_movement(movement);
        let tmp_score = -evaluate_critic(&tmp_board, depth - 1, tmp_prune);
        
        if tmp_score >= prune.beta {
            return prune.beta;
        }

        if tmp_score + 200 < prune.alpha {
            return prune.alpha;
        }

        if tmp_score > prune.alpha {
            prune.alpha = tmp_score;
        }
    }

    prune.alpha
}

fn estimate_movement(movement: &Movement) -> i32 {
    let mut score = 0;

    score -= piece_value(movement.moved);

    if let Some(captured) = movement.captured {
        score += 10 * piece_value(captured);
    }

    if let Some(promotion) = movement.promotion {
        score += 20 * piece_value(promotion);
    }

    score
}

fn count_material(board: &Board) -> i32 {
    let mut score: i32 = 0;
    
    for piece in board.ocuppied_squares() {            
        if piece.color == board.active_color {
            score = score + piece_value(piece.kind);
        } else {
            score = score - piece_value(piece.kind);
        }
    }

    score
}

fn piece_value(piece_kind: PieceKind) -> i32 {
    match piece_kind {
        PieceKind::Pawn => 100,
        PieceKind::Bishop => 300,
        PieceKind::Knight => 300,
        PieceKind::Rook => 500,
        PieceKind::Queen => 900,
        PieceKind::King => 1_000_000,
        PieceKind::Duck => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::Position;

    #[test]
    fn test_obvious() {
        let board = Board::from_fen("4k3/8/5r2/2KN4/8/8/8/8 w - - 0 1");
        let best_move = search(&board, 2);
        
        if let None = best_move {
            panic!("No moves found");
        }
        let best_move = best_move.unwrap();
        
        assert_eq!(best_move.origin, Position(3, 4));
        assert_eq!(best_move.target, Position(5, 5));
    }
    
    #[test]
    fn test_forks() {
        let board = Board::from_fen("4k3/8/4r3/2KN4/8/8/8/8 w - - 0 1");
        let best_move = search(&board, 4);

        if let None = best_move {
            panic!("No moves found");
        }
        let best_move = best_move.unwrap();

        assert_eq!(best_move.origin, Position(3, 4));
        assert_eq!(best_move.target, Position(2, 6));
    }
}