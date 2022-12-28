use crate::board::Board;
use crate::pieces::PieceKind;
use crate::pieces::Position;
use crate::pieces::Piece;
use crate::pieces::Color;

#[derive(Copy, Clone, Debug)]
pub struct Movement {
    pub origin: Position,
    pub target: Position,
}

#[derive(Debug)]
enum MoveKind {
    Simple,
    Capture,
    Invalid,
}

impl Movement {
    pub fn avaliable_moves(board: &Board) -> Vec::<Movement>{
        let mut movements = Vec::<Movement>::with_capacity(140);
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

    pub fn piece_moves(board: &Board, origin: Position) -> Vec::<Movement> {
        let piece = match board.get_square(origin.0 as usize, origin.1 as usize).piece {
            Some(p) => p,
            None => return Vec::<Movement>::new(), // goes out if None
        };
        
        match piece.kind {
            PieceKind::Rook => Self::rook_moves(board, origin),
            PieceKind::Bishop => Self::bishop_moves(board, origin),
            PieceKind::Queen => Self::queen_moves(board, origin),
            PieceKind::King => Self::king_moves(board, origin),
            PieceKind::Knight => Self::knight_moves(board, origin),
            PieceKind::Pawn => Self::pawn_moves(board, origin),
        }
    }

    fn rook_moves(board: &Board, origin: Position) -> Vec::<Movement> {
        let mut movements = Vec::<Movement>::new();
        
        let start = origin.0 + 1;
        let end = 8;
        for i in start..end {
            let target = Position(i, origin.1);
            let movement = Movement{origin, target};
            match Self::check_move(board, movement) {
                MoveKind::Simple => movements.push(movement),
                MoveKind::Capture => {movements.push(movement); break},
                MoveKind::Invalid => break,
            }
        }
                
        let start = 0;
        let end = origin.0;
        for i in (start..end).rev() {
            let target = Position(i, origin.1);
            let movement = Movement{origin, target};
            
            match Self::check_move(board, movement) {
                MoveKind::Simple => movements.push(movement),
                MoveKind::Capture => {movements.push(movement); break},
                MoveKind::Invalid => break,
            }
        }

        let start = origin.1 + 1;
        let end = 8;
        for i in start..end {
            let target = Position(origin.0, i);
            let movement = Movement{origin, target};
            
            match Self::check_move(board, movement) {
                MoveKind::Simple => movements.push(movement),
                MoveKind::Capture => {movements.push(movement); break},
                MoveKind::Invalid => break,
            }
        }
        
        let start = 0;
        let end = origin.1;
        for i in (start..end).rev() {
            let target = Position(origin.0, i);
            let movement = Movement{origin, target};
            
            match Self::check_move(board, movement) {
                MoveKind::Simple => movements.push(movement),
                MoveKind::Capture => {movements.push(movement); break},
                MoveKind::Invalid => break,
            }
        }
        
        movements
    }

    fn bishop_moves(board: &Board, origin: Position) -> Vec::<Movement> {
        let mut movements = Vec::<Movement>::new();
        
        // I dont care about the boundaries
        // if it is out, the movement will be Invalid
        
        for i in 1..8 {
            let target = Position(origin.0 + i, origin.1 + i);
            let movement = Movement{origin, target};
            
            match Self::check_move(board, movement) {
                MoveKind::Simple => movements.push(movement),
                MoveKind::Capture => {movements.push(movement); break},
                MoveKind::Invalid => break,
            };
        }
        
        for i in 1..8 {
            let target = Position(origin.0 + i, origin.1 - i);
            let movement = Movement{origin, target};
            
            match Self::check_move(board, movement) {
                MoveKind::Simple => movements.push(movement),
                MoveKind::Capture => {movements.push(movement); break},
                MoveKind::Invalid => break,
            };
        }
        
        for i in 1..8 {
            let target = Position(origin.0 - i, origin.1 + i);
            let movement = Movement{origin, target};
            
            match Self::check_move(board, movement) {
                MoveKind::Simple => movements.push(movement),
                MoveKind::Capture => {movements.push(movement); break},
                MoveKind::Invalid => break,
            };
        }

        for i in 1..8 {
            let target = Position(origin.0 - i, origin.1 - i);
            let movement = Movement{origin, target};

            match Self::check_move(board, movement) {
                MoveKind::Simple => movements.push(movement),
                MoveKind::Capture => {movements.push(movement); break},
                MoveKind::Invalid => break,
            };
        }

        movements
    }

    fn queen_moves(board: &Board, origin: Position) -> Vec::<Movement> {
        let rook = Self::rook_moves(board, origin);
        let bishop = Self::bishop_moves(board, origin);
        [rook, bishop].concat()
    }

    fn king_moves(board: &Board, origin: Position) -> Vec::<Movement> {
        let mut movements = Vec::<Movement>::new();
        let existing_moves = [
            Position(origin.0 - 1, origin.1 - 1),
            Position(origin.0 - 1, origin.1),
            Position(origin.0 - 1, origin.1 + 1),
            
            Position(origin.0,  origin.1 - 1),
            Position(origin.0,  origin.1 + 1),
            
            Position(origin.0 + 1, origin.1 - 1),
            Position(origin.0 + 1, origin.1),
            Position(origin.0 + 1, origin.1 + 1),
        ];

        for target in existing_moves {
            let movement = Movement{origin, target};
            
            match Self::check_move(board, movement) {
                MoveKind::Simple => movements.push(movement),
                MoveKind::Capture => movements.push(movement),
                MoveKind::Invalid => (),
            };
        }
        
        movements
    }

    fn knight_moves(board: &Board, origin: Position) -> Vec::<Movement> {
        let mut movements = Vec::<Movement>::new();
        let existing_moves = [
            Position(origin.0 - 1, origin.1 - 2),
            Position(origin.0 - 1, origin.1 + 2),
            
            Position(origin.0 - 2, origin.1 - 1),
            Position(origin.0 - 2, origin.1 + 1),
            
            Position(origin.0 + 1, origin.1 - 2),
            Position(origin.0 + 1, origin.1 + 2),
            
            Position(origin.0 + 2, origin.1 - 1),
            Position(origin.0 + 2, origin.1 + 1),
            ];
            
        for target in existing_moves {
            let movement = Movement{origin, target};
            
            match Self::check_move(board, movement) {
                MoveKind::Simple => movements.push(movement),
                MoveKind::Capture => movements.push(movement),
                MoveKind::Invalid => (),
            };
        }
        
        movements
    }

    fn pawn_moves(board: &Board, origin: Position) -> Vec::<Movement> {
        match board.get_square(origin.0 as usize, origin.1 as usize).piece {
            Some(Piece{color:Color::White, kind:_}) => Self::white_pawn_moves(board, origin),
            Some(Piece{color:Color::Black, kind:_}) => Self::black_pawn_moves(board, origin),
            None => Vec::<Movement>::new(),
        }
    }

    fn white_pawn_moves(board: &Board, origin: Position) -> Vec::<Movement> {
        let mut movements = Vec::<Movement>::new();
        
        let move_ahead = Movement{origin, target:Position(origin.0, origin.1 + 1)};
        let double_move = Movement{origin, target:Position(origin.0, origin.1 + 2)};
        let take_left = Movement{origin, target:Position(origin.0 - 1, origin.1 + 1)};
        let take_right = Movement{origin, target:Position(origin.0 + 1, origin.1 + 1)};
        
        if let MoveKind::Simple = Self::check_move(board, move_ahead) {
            movements.push(move_ahead);

            if origin.1 == 1 {
                if let MoveKind::Simple = Self::check_move(board, double_move) {
                    movements.push(double_move);
                }
            }
        }
        
        if let MoveKind::Capture = Self::check_move(board, take_left) {
            movements.push(take_left);
        }
        
        if let MoveKind::Capture = Self::check_move(board, take_right) {
            movements.push(take_right);
        }
        
        if board.en_passant {
            if let Some(movement) = board.last_move {
                if let MoveKind::Simple = Self::check_move(board, take_left) {
                    if movement.target.0 == (origin.0 - 1) && movement.target.1 == origin.1 {
                        movements.push(take_left);
                    }
                }
                
                if let MoveKind::Simple = Self::check_move(board, take_right) {
                    if movement.target.0 == (origin.0 + 1) && movement.target.1 == origin.1 {
                        movements.push(take_right);
                    }
                }
            }
        }

        movements
    }

    fn black_pawn_moves(board: &Board, origin: Position) -> Vec::<Movement> {
        let mut movements = Vec::<Movement>::new();
        
        let move_ahead = Movement{origin, target:Position(origin.0, origin.1 - 1)};
        let double_move = Movement{origin, target:Position(origin.0, origin.1 - 2)};
        let take_left = Movement{origin, target:Position(origin.0 - 1, origin.1 - 1)};
        let take_right = Movement{origin, target:Position(origin.0 + 1, origin.1 - 1)};
        
        if let MoveKind::Simple = Self::check_move(board, move_ahead) {
            movements.push(move_ahead);
            
            if origin.1 == 6 {
                if let MoveKind::Simple = Self::check_move(board, double_move) {
                    movements.push(double_move);
                }
            }
        }
        
        if let MoveKind::Capture = Self::check_move(board, take_left) {
            movements.push(take_left);
        }

        if let MoveKind::Capture = Self::check_move(board, take_right) {
            movements.push(take_right);
        }
        
        if board.en_passant {
            if let Some(movement) = board.last_move {
                if let MoveKind::Simple = Self::check_move(board, take_left) {
                    if movement.target.0 == (origin.0 - 1) && movement.target.1 == origin.1 {
                        movements.push(take_left);
                    }
                }
                
                if let MoveKind::Simple = Self::check_move(board, take_right) {
                    if movement.target.0 == (origin.0 + 1) && movement.target.1 == origin.1 {
                        movements.push(take_right);
                    }
                }
            }
        }
        
        movements
    }

    fn check_move(board: &Board, movement: Movement) -> MoveKind {
        // Helper to check if we made a capture, simple movement or invalid movement
        
        if movement.origin.0 < 0 || movement.origin.1 < 0 {
            return MoveKind::Invalid;
        }
        
        if movement.origin.0 >= 8 || movement.origin.1 >= 8 {
            return MoveKind::Invalid;
        }
        
        if movement.target.0 < 0 || movement.target.1 < 0 {
            return MoveKind::Invalid;
        }
        
        if movement.target.0 >= 8 || movement.target.1 >= 8 {
            return MoveKind::Invalid;
        }

        let origin_piece = match board.get_square(movement.origin.0 as usize, movement.origin.1 as usize).piece {
            Some(p) => p,
            None => return MoveKind::Invalid,
        };
        
        if let Some(target_square) = board.get_square(movement.target.0 as usize, movement.target.1 as usize).piece {
            if origin_piece.color == target_square.color {
                return MoveKind::Invalid;
            } else {
                return MoveKind::Capture;
            }
        } else {
            return MoveKind::Simple;
        }
    }
}