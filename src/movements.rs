use crate::board::Board;
use crate::pieces::PieceKind;
use crate::pieces::Position;
use crate::pieces::Piece;
use crate::pieces::Color;

#[derive(Copy, Clone, Debug)]
pub struct Movement {
    pub origin: Position,
    pub target: Position,
    pub moved: PieceKind,
    pub captured: Option<PieceKind>,
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
    fn from_coords(board: &Board, x0: i32, y0: i32, x1: i32, y1: i32) -> Option<Self> {
        if !Self::in_boundaries(x0, y0) || !Self::in_boundaries(x1, y1) {
            return None;
        }

        // if it is in boundaries we can safely convert to usize
        let (x0, y0, x1, y1) = (x0 as usize, y0 as usize, x1 as usize, y1 as usize);

        let origin_square = board.get_square(x0, y0);
        let target_square = board.get_square(x1, y1);

        // if we can not unwrap, the movement is invalid anyway
        let origin_piece = origin_square.piece.unwrap();
        let captured = match target_square.piece {
            Some(piece) => Some(piece.kind),
            None => None
        };

        // you cant capture your own pieces
        if let Some(target_piece) = target_square.piece {
            if origin_piece.color == target_piece.color {
                return None;
            }
        }

        // TODO: change position values to usize
        let movement = Movement {
            origin: Position(x0 as i32, y0 as i32),
            target: Position(x1 as i32, y1 as i32),
            moved: origin_piece.kind,
            captured,
            promotion: None,
        };

        Some(movement)
    }

    pub fn avaliable_moves(board: &Board) -> Vec::<Self>{
        let mut movements = Vec::<Self>::with_capacity(140);
        let mut king_found = false;

        for square in board.ocuppied_squares() {
            let piece = square.piece.unwrap();
            
            if piece.color == board.active_color {
                let mut piece_movements = Self::piece_moves(board, square.pos);
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
        let piece = board.get_square(origin.0 as usize, origin.1 as usize).piece;

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
        [Self::rook_moves(board, origin), Self::bishop_moves(board, origin)].concat()
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
        
        let direction: i32;
        let promotion: i32;

        match board.get_square(origin.0 as usize, origin.1 as usize).piece {
            Some(Piece{color:Color::White, kind:_}) => {direction = 1; promotion = 7},
            Some(Piece{color:Color::Black, kind:_}) => {direction = -1; promotion = 0},
            None => return movements,
        }

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
            let mut tmp_movement = movement.clone();
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
            MovementDirection::BottomLeft => (-1, 1),
            MovementDirection::BottomRight => (1, 1),
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