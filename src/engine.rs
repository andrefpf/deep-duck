use std::cmp::Ordering;

use crate::board::Board;
use crate::pieces::PieceKind;
use crate::pieces::Position;
use crate::movements::Movement;
use crate::evaluation::{count_centipawns, piece_value};


#[derive(Copy, Clone, Debug)]
struct Prune {
    alpha: i32,
    beta: i32,
}

#[derive(Copy, Clone, Debug)]
struct Evaluation {
    movement: Option<Movement>,
    score: i32,
}

impl Prune {
    fn invert(&self) -> Self {
        Prune{
            alpha: -self.beta, 
            beta: -self.alpha, 
        }
    }
}

pub fn search(board: &Board, depth: usize) -> Option<Movement> {
    let prune = Prune {
        alpha: -i32::MAX,
        beta: i32::MAX,
    };
    _search(board, depth, prune).movement
}

fn _search(board: &Board, depth: usize, prune: Prune) -> Evaluation {
    if depth == 0 {
        return _evaluate(board);
    }

    let mut best = Evaluation { movement: None, score:  -i32::MAX };
    let mut prune = prune;
    let mut simple_movements = Movement::avaliable_moves(board);
    simple_movements.sort_by_cached_key(|x| -estimate_movement(x));

    if simple_movements.is_empty() {
        return _evaluate(board);
    }

    for movement in simple_movements {
        let evaluation = duck_search(board, depth-1, prune.invert(), movement);

        if evaluation.score >= prune.beta {
            return Evaluation{movement: evaluation.movement, score: prune.beta};
        }

        if evaluation.score > prune.alpha {
            prune.alpha = evaluation.score;
            best = evaluation;
        }
    }

    best
}

fn _evaluate(board: &Board) -> Evaluation {
    Evaluation {
        movement: None,
        score: count_centipawns(board),
    }
}

fn duck_search(board: &Board, depth: usize, prune: Prune, movement: Movement) -> Evaluation {
    let mut best = movement;
    let mut tmp_board = board.copy_movement(movement);
    let mut threat = _search(&tmp_board, depth, prune);

    if let Some(reaction) = threat.movement {
        for duck_target in intercept(board, &reaction) {
            let alternative_movement = Movement {duck_target, ..movement};
            tmp_board = board.copy_movement(alternative_movement);
            let alternative_threat = _search(&tmp_board, depth, prune);
            
            if alternative_threat.score < threat.score {
                threat = alternative_threat;
                best = alternative_movement;
            }
        }
    }

    Evaluation {
        movement: Some(best),
        score: -threat.score,
    }
}

fn estimate_movement(movement: &Movement) -> i32 {
    let mut score = 0;
    let Position(x, y) = movement.target;

    score -= piece_value(movement.moved);
    score += x*(7-x) + y*(7-y);

    if let Some(captured) = movement.captured {
        score += 10 * piece_value(captured.kind);
    }

    if let Some(promotion) = movement.promotion {
        score += 20 * piece_value(promotion);
    }

    score
}

fn intercept(board: &Board, threat: &Movement) -> Vec<Position> {
    let duck = match threat.moved {
        PieceKind::Knight | PieceKind::King | PieceKind::Pawn => {
            intercept_jump(threat)
        },
        
        PieceKind::Rook | PieceKind::Bishop | PieceKind::Queen => {
            intercept_slide(board, threat)
        },
        
        PieceKind::Duck => {
            None
        },
    };
    
    let mut ducks = Vec::<Position>::new();

    if board.get_square(threat.duck_target).is_none() {
        ducks.push(threat.duck_target);
    }
    
    if let Some(duck_pos) = duck {
        ducks.push(duck_pos);
    }

    ducks
}

fn intercept_jump(movement: &Movement) -> Option<Position> {
    if movement.captured.is_none() {
        Some(movement.target)
    } else {
        None
    }
}

fn intercept_slide(board: &Board, movement: &Movement) -> Option<Position> {
    let dx = match (movement.target.0 - movement.origin.0).cmp(&0) {
        Ordering::Greater => 1,
        Ordering::Less => -1,
        Ordering::Equal => 0,
    };

    let dy = match (movement.target.1 - movement.origin.1).cmp(&0) {
        Ordering::Greater => 1,
        Ordering::Less => -1,
        Ordering::Equal => 0,
    };

    // Usually you have 2 ways to block a movement:
    // puting the duck next to the atacker (best)
    // or puting the duck next to the victm (if the other option already has a duck)
    let pos_1 = Position(movement.origin.0 + dx, movement.origin.1 + dy);
    let pos_2 = Position(movement.target.0 - dx, movement.target.1 - dy);

    if board.get_square(pos_1).is_none() {
        Some(pos_1)
    } else if board.get_square(pos_2).is_none() {
        Some(pos_2)
    } else {
        None
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
        
        if best_move.is_none() {
            panic!("No moves found");
        }
        let best_move = best_move.unwrap();
        
        assert_eq!(best_move.origin, Position(3, 4));
        assert_eq!(best_move.target, Position(5, 5));
    }
    
    #[test]
    fn test_forks() {
        let board = Board::from_fen("4k3/8/4q3/2KN4/8/8/8/8 w - - 0 1");
        let best_move = search(&board, 4);
        println!("{:?}", board);
        
        if best_move.is_none() {
            panic!("No moves found");
        }
        let best_move = best_move.unwrap();
        println!("{:?}", board);
        
        assert_eq!(best_move.origin, Position(3, 4));
        assert_eq!(best_move.target, Position(2, 6));
    }

    // Some of the tactics in these tests are from Eric Rosen's video
    // avaliable in https://www.youtube.com/watch?v=Xeil4C9rU34

    #[test]
    fn test_ducktics_1() {
        let board = Board::from_fen("8/3*4/8/8/8/4K3/8/7k w - - 0 1 q");
        let best_move = search(&board, 4);

        if best_move.is_none() {
            panic!("No moves found");
        }
        let best_move = best_move.unwrap();

        assert_eq!(best_move.origin, Position(4, 2));
        assert_eq!(best_move.target, Position(5, 1));
        assert_eq!(best_move.duck_target, Position(7, 1));
    }

    #[test]
    fn test_ducktics_2() {
        let board = Board::from_fen("3r3r/pp6/2pk1pp1/3p4/5P1p/P5nP/1P4PK/2RB*q2 b - - 0 1 q");
        let best_move = search(&board, 4);

        if best_move.is_none() {
            panic!("No moves found");
        }
        let best_move = best_move.unwrap();

        assert_eq!(best_move.origin, Position(5, 0));
        assert_eq!(best_move.target, Position(7, 0));
    }

    #[test]
    fn test_ducktics_3() {
        let board = Board::from_fen("r4rk1/p4ppp/2nb4/3p4/3q*1n1/1PN4P/1BP2PP1/3RQRK1 b - - 0 1 q");
        let best_move = search(&board, 4);

        if best_move.is_none() {
            panic!("No moves found");
        }
        let best_move = best_move.unwrap();

        assert_eq!(best_move.origin, Position(3, 5));
        assert_eq!(best_move.target, Position(7, 1));
        assert_eq!(best_move.duck_target, Position(7, 0));
    }

    #[test]
    fn test_ducktics_4() {
        let board = Board::from_fen("2kr4/p4p2/2p2p2/2p5/1PP5/2b2q2/P4*2/5KR1 b - - 0 1 q");
        let best_move = search(&board, 4);

        if best_move.is_none() {
            panic!("No moves found");
        }
        let best_move = best_move.unwrap();

        assert_eq!(best_move.origin, Position(3, 7));
        assert_eq!(best_move.target, Position(3, 0));
    }

    #[test]
    fn test_ducktics_5() {
        let board = Board::from_fen("6k1/5p2/6*1/6QP/4B3/8/5P2/5K2 w - - 0 1 q");
        let best_move = dbg!(search(&board, 4));

        if best_move.is_none() {
            panic!("No moves found");
        }
        let best_move = best_move.unwrap();

        assert_eq!(best_move.origin, Position(6, 4));
        assert_eq!(best_move.target, Position(5, 5));
        assert_eq!(best_move.duck_target, Position(5, 7));
    }

    #[test]
    fn test_ducktics_6() {
        let board = Board::from_fen("kb*5/p2B4/8/8/8/8/8/7K w - - 0 1");
        let best_move = dbg!(search(&board, 4));

        if best_move.is_none() {
            panic!("No moves found");
        }
        let best_move = best_move.unwrap();

        assert_eq!(best_move.origin, Position(3, 6));
        assert_eq!(best_move.target, Position(2, 5));
        assert_eq!(best_move.duck_target, Position(1, 6));
    }

    #[test]
    fn test_ducktics_7() {
        let board = Board::from_fen("rnbqkb1r/ppp1pppp/2*2n2/1B1p3Q/4P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 1");
        let best_move = dbg!(search(&board, 4));

        if best_move.is_none() {
            panic!("No moves found");
        }
        let best_move = best_move.unwrap();

        assert_eq!(best_move.origin, Position(5, 5));
        assert_eq!(best_move.target, Position(7, 4));
        assert_eq!(best_move.duck_target, Position(3, 6));
    }
}