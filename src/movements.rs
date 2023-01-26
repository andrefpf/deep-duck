use crate::board::Board;
use crate::pieces::PieceKind;
use crate::pieces::Position;
use crate::pieces::Piece;
use crate::pieces::Color;


#[derive(Copy, Clone, Debug)]
pub struct Movement {
    pub origin: Position,
    pub target: Position,
    pub duck_origin: Option<Position>,
    pub duck_target: Position,
    pub color: Color,
    pub moved: PieceKind,
    pub captured: Option<Piece>,
    pub promotion: Option<PieceKind>
}

enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
    UpperLeft,
    UpperRight,
    BottomLeft,
    BottomRight,
}

impl Movement {
    pub fn from_coords(board: &Board, x0: i32, y0: i32, x1: i32, y1: i32) -> Option<Self> {
        if !Self::in_boundaries(x0, y0) || !Self::in_boundaries(x1, y1) {
            return None;
        }

        let origin = Position(x0, y0);
        let target = Position(x1, y1);

        let origin_square = board.get_square(origin);
        let target_square = board.get_square(target);

        // if we can not unwrap, the movement is invalid anyway
        let origin_piece = origin_square.unwrap();

        if let Some(target_piece) = target_square {
            // you cant capture your own pieces
            if origin_piece.color == target_piece.color {
                return None;
            }

            // you cant capture the duck
            if let Color::Yellow = target_piece.color {
                return None
            }
        }

        // TODO: change position values to usize
        let movement = Movement {
            origin,
            target,
            duck_origin: board.duck,
            duck_target: origin,
            color: origin_piece.color,
            moved: origin_piece.kind,
            captured: target_square,
            promotion: None,
        };

        Some(movement)
    }

    pub fn try_movement(board: &Board, origin: Position, target: Position, duck: Position) -> Option<Self> {
        let origin_piece = match board.get_square(origin) {
            Some(piece) => piece,
            None => return None,
        };
        
        if (duck != origin) && board.get_square(duck).is_some() {
            return None;
        }

        if duck == target {
            return None;
        }

        if origin_piece.color != board.active_color {
            return None;
        }

        for movement in Movement::piece_moves(board, origin) {
            if movement.origin == origin && movement.target == target {
                let (x0, y0, x1, y1) = (origin.0, origin.1, target.0, target.1);
                if let Some(mut movement) = Movement::from_coords(board, x0, y0, x1, y1) {
                    movement.duck_target = duck;
                    return Some(movement);
                }
            }
        }
        
        None
    }

    pub fn avaliable_moves(board: &Board) -> Vec::<Self>{
        let mut movements = Vec::<Self>::with_capacity(140);
        let mut king_found = false;

        for piece in board.ocuppied_squares() {            
            if piece.color == board.active_color {
                let mut piece_movements = Self::piece_moves(board, piece.pos);
                movements.append(&mut piece_movements);
                
                if let PieceKind::King = piece.kind {
                    king_found = true;
                }
            }
        }

        if !king_found {
            movements.clear();
        }

        movements
    }

    pub fn piece_moves(board: &Board, origin: Position) -> Vec::<Self> {
        let piece = board.get_square(origin);

        if piece.is_none() {
            return Vec::<Self>::new();
        }
        
        match piece.unwrap().kind {
            PieceKind::Rook => Self::rook_moves(board, origin),
            PieceKind::Bishop => Self::bishop_moves(board, origin),
            PieceKind::Queen => Self::queen_moves(board, origin),
            PieceKind::King => Self::king_moves(board, origin),
            PieceKind::Knight => Self::knight_moves(board, origin),
            PieceKind::Pawn => Self::pawn_moves(board, origin),
            PieceKind::Duck => Vec::<Self>::new(),
        }
    }

    fn rook_moves(board: &Board, origin: Position) -> Vec<Self> {
        [
            Self::slide_movements(board, origin, MovementDirection::Up),
            Self::slide_movements(board, origin, MovementDirection::Down),
            Self::slide_movements(board, origin, MovementDirection::Left),
            Self::slide_movements(board, origin, MovementDirection::Right),
        ].concat()
    }

    fn bishop_moves(board: &Board, origin: Position) -> Vec<Self> {
        [
            Self::slide_movements(board, origin, MovementDirection::UpperLeft),
            Self::slide_movements(board, origin, MovementDirection::UpperRight),
            Self::slide_movements(board, origin, MovementDirection::BottomLeft),
            Self::slide_movements(board, origin, MovementDirection::BottomRight),
        ].concat()
    }

    fn queen_moves(board: &Board, origin: Position) -> Vec<Self> {
        [
            Self::slide_movements(board, origin, MovementDirection::Up),
            Self::slide_movements(board, origin, MovementDirection::Down),
            Self::slide_movements(board, origin, MovementDirection::Left),
            Self::slide_movements(board, origin, MovementDirection::Right),
            Self::slide_movements(board, origin, MovementDirection::UpperLeft),
            Self::slide_movements(board, origin, MovementDirection::UpperRight),
            Self::slide_movements(board, origin, MovementDirection::BottomLeft),
            Self::slide_movements(board, origin, MovementDirection::BottomRight),
        ].concat()
    }

    fn king_moves(board: &Board, origin: Position) -> Vec<Self> {
        let displacement = Vec::from([
            (-1, 1), (0, 1), (1, 1), 
            (-1, 0), (1, 0), 
            (-1, -1), (0, -1), (1, -1), 
        ]);
        Self::jump_movements(board, origin, displacement)
    }

    fn knight_moves(board: &Board, origin: Position) -> Vec<Self> {
        let displacement = Vec::from([
            (-1, 2),
            (-1, -2),
            (-2, 1),
            (-2, -1),
            (1, 2),
            (1, -2),
            (2, 1),
            (2, -1),
        ]);
        Self::jump_movements(board, origin, displacement)
    }

    fn pawn_moves(board: &Board, origin: Position) -> Vec<Self> {
        let mut movements = Vec::<Movement>::new();
        
        let (direction, promotion) = match board.get_square(origin) {
            Some(Piece{pos:_, color:Color::White, kind:PieceKind::Pawn}) => (1, 7),
            Some(Piece{pos:_, color:Color::Black, kind:PieceKind::Pawn}) => (-1, 0),
            _ => return movements,
        };

        let Position(x, y) = origin;

        if let Some(movement) = Self::from_coords(board, x, y, x, y + direction) {
            if movement.captured.is_none() {
                if y + direction == promotion {
                    movements.append(&mut Self::promotions(&movement));
                } else {
                    movements.push(movement);
                }
            }
        }

        if let Some(movement) = Self::from_coords(board, x, y, x - 1, y + direction) {
            if movement.captured.is_some() {
                if y + direction == promotion {
                    movements.append(&mut Self::promotions(&movement));
                } else {
                    movements.push(movement);
                }
            }
        }

        if let Some(movement) = Self::from_coords(board, x, y, x + 1, y + direction) {
            if movement.captured.is_some() {
                if y + direction == promotion {
                    movements.append(&mut Self::promotions(&movement));
                } else {
                    movements.push(movement);
                }
            }
        }

        // first double move
        if y == promotion - 6*direction {
            let try_movement = Self::from_coords(board, x, y, x, y + direction);
            let can_move_one = try_movement.is_some() && try_movement.unwrap().captured.is_none();

            if can_move_one {
                if let Some(movement) = Self::from_coords(board, x, y, x, y + 2*direction) {
                    if movement.captured.is_none() {
                        movements.push(movement);
                    }        
                }    
            }
        }

        movements
    }

    fn promotions(movement: &Movement) -> Vec<Self> {
        let mut movements = Vec::<Movement>::new();
        let promotions = [PieceKind::Knight, PieceKind::Queen]; // the only reasonable promotions
        // let promotions = [PieceKind::Rook, PieceKind::Knight, PieceKind::Bishop, PieceKind::Queen];

        for kind in promotions {
            let mut tmp_movement = *movement;
            tmp_movement.promotion = Some(kind);
            movements.push(tmp_movement);
        }

        movements
    }

    fn in_boundaries(x: i32, y: i32) -> bool {
        (0..8).contains(&x) && (0..8).contains(&y)
    }

    fn slide_movements(board: &Board, origin: Position, direction: MovementDirection) -> Vec::<Self> {
        let (dx, dy) = match direction {
            MovementDirection::Up => (0, 1),
            MovementDirection::Down => (0, -1),
            MovementDirection::Left => (-1, 0),
            MovementDirection::Right => (1, 0),
            MovementDirection::UpperLeft => (-1, 1),
            MovementDirection::UpperRight => (1, 1),
            MovementDirection::BottomLeft => (-1, -1),
            MovementDirection::BottomRight => (1, -1),
        };

        let mut movements = Vec::<Movement>::new();

        for i in 1..8 {
            let Position(x, y) = origin;
            let try_movement = Self::from_coords(board, x, y, x + i*dx, y + i*dy);

            if let Some(movement) = try_movement {  
                movements.push(movement);
                if movement.captured.is_some() {
                    break;
                }
            } else {
                break
            }
        }

        movements
    }

    fn jump_movements(board: &Board, origin: Position, displacement: Vec<(i32, i32)>) -> Vec::<Movement> {
        let mut movements = Vec::<Movement>::new();

        for (dx, dy) in displacement {
            let Position(x, y) = origin;
            let try_movement = Self::from_coords(board, x, y, x + dx, y + dy);

            if let Some(movement) = try_movement {
                movements.push(movement);
            }
        }

        movements
    }
}


#[allow(dead_code)]
pub fn perft(board: &mut Board, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;

    for movement in Movement::avaliable_moves(board) {
        board.make_movement(movement);
        nodes += perft(board, depth-1);
        board.unmake_movement(movement);

        // let mut tmp_board = board.copy_movement(movement);
        // nodes += perft(&mut tmp_board, depth-1);
    }
    
    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perft() {
        // https://www.chessprogramming.org/Perft_Results
        let nodes = perft(&mut Board::arranged(), 3);    
        assert!(nodes == 8902);
    }

    #[test]
    fn test_avaliable_white() {
        let board = Board::from_fen("k6B/1P6/8/7R/8/1r6/P7/K5QN w - - 0 1");
        let avaliable = Movement::avaliable_moves(&board);
        assert_eq!(avaliable.len(), 48);
    }
    
    #[test]
    fn test_avaliable_black() {
        let board = Board::from_fen("4k3/1p6/5r2/2KN4/8/2p5/1PPP4/8 b - - 0 1");
        let avaliable = Movement::avaliable_moves(&board);
        assert_eq!(avaliable.len(), 23);
    }
}