use crate::board::Board;
use crate::pieces::PieceKind;
use crate::pieces::Position;
use crate::pieces::Piece;
use crate::pieces::Color;

#[derive(Copy, Clone, Debug)]
pub struct Movement {
    origin: Position,
    target: Position,
}

#[derive(Debug)]
enum MoveKind {
    Simple,
    Capture,
    Invalid,
}

pub fn piece_moves(board: &Board, origin: Position) -> Vec::<Movement> {
    let piece = match board.get_piece(origin) {
        Some(p) => p,
        None => return Vec::<Movement>::new(), // goes out if None
    };
    
    match piece.kind {
        PieceKind::Rook => rook_moves(board, origin),
        PieceKind::Bishop => bishop_moves(board, origin),
        PieceKind::Queen => queen_moves(board, origin),
        PieceKind::King => king_moves(board, origin),
        PieceKind::Knight => knight_moves(board, origin),
        _ => Vec::<Movement>::new(),
    }
}

fn rook_moves(board: &Board, origin: Position) -> Vec::<Movement> {
    let mut movements = Vec::<Movement>::new();

    let start = origin.0 + 1;
    let end = 8;
    for i in start..end {
        let target = Position(i, origin.1);
        let movement = Movement{origin, target};
        match check_move(board, movement) {
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

        match check_move(board, movement) {
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

        match check_move(board, movement) {
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

        match check_move(board, movement) {
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

        match check_move(board, movement) {
            MoveKind::Simple => movements.push(movement),
            MoveKind::Capture => {movements.push(movement); break},
            MoveKind::Invalid => break,
        };
    }

    for i in 1..8 {
        let target = Position(origin.0 + i, origin.1 - i);
        let movement = Movement{origin, target};

        match check_move(board, movement) {
            MoveKind::Simple => movements.push(movement),
            MoveKind::Capture => {movements.push(movement); break},
            MoveKind::Invalid => break,
        };
    }

    for i in 1..8 {
        let target = Position(origin.0 - i, origin.1 + i);
        let movement = Movement{origin, target};

        match check_move(board, movement) {
            MoveKind::Simple => movements.push(movement),
            MoveKind::Capture => {movements.push(movement); break},
            MoveKind::Invalid => break,
        };
    }

    for i in 1..8 {
        let target = Position(origin.0 - i, origin.1 - i);
        let movement = Movement{origin, target};

        match check_move(board, movement) {
            MoveKind::Simple => movements.push(movement),
            MoveKind::Capture => {movements.push(movement); break},
            MoveKind::Invalid => break,
        };
    }

    movements
}

fn queen_moves(board: &Board, origin: Position) -> Vec::<Movement> {
    let rook = rook_moves(board, origin);
    let bishop = bishop_moves(board, origin);
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

        match check_move(board, movement) {
            MoveKind::Simple => movements.push(movement),
            MoveKind::Capture => {movements.push(movement); break},
            MoveKind::Invalid => break,
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

        Position(origin.0 + 1, origin.1 - 1),
        Position(origin.0 + 1, origin.1 + 1),

        Position(origin.0 + 2, origin.1 - 2),
        Position(origin.0 + 2, origin.1 + 2),
    ];

    for target in existing_moves {
        let movement = Movement{origin, target};

        match check_move(board, movement) {
            MoveKind::Simple => movements.push(movement),
            MoveKind::Capture => {movements.push(movement); break},
            MoveKind::Invalid => break,
        };
    }

    movements
}

fn pawn_moves(board: &Board, origin: Position) -> Vec::<Movement> {
    match board.get_piece(origin) {
        Some(Piece{color:Color::White, kind:_}) => white_pawn_moves(board, origin),
        Some(Piece{color:Color::Black, kind:_}) => black_pawn_moves(board, origin),
        None => Vec::<Movement>::new(),
    }
}

fn white_pawn_moves(board: &Board, origin: Position) -> Vec::<Movement> {
    let mut movements = Vec::<Movement>::new();

    let move_ahead = Movement{origin, target:Position(origin.0, origin.1 + 1)};
    let double_move = Movement{origin, target:Position(origin.0, origin.1 + 2)};
    let take_left = Movement{origin, target:Position(origin.0 - 1, origin.1 + 1)};
    let take_right = Movement{origin, target:Position(origin.0 + 1, origin.1 + 1)};

    if let MoveKind::Simple = check_move(board, move_ahead) {
        movements.push(move_ahead);

        if origin.1 == 1 {
            if let MoveKind::Simple = check_move(board, double_move) {
                movements.push(double_move);
            }
        }
    }

    if let MoveKind::Capture = check_move(board, take_left) {
        movements.push(take_left);
    }

    if let MoveKind::Capture = check_move(board, take_right) {
        movements.push(take_right);
    }

    if let Some(position) = board.en_passant {
        if let MoveKind::Simple = check_move(board, take_left) {
            if position.0 == (origin.0 - 1) && position.1 == origin.1 {
                movements.push(take_left);
            }
        }

        if let MoveKind::Simple = check_move(board, take_right) {
            if position.0 == (origin.0 + 1) && position.1 == origin.1 {
                movements.push(take_right);
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

    if let MoveKind::Simple = check_move(board, move_ahead) {
        movements.push(move_ahead);

        if origin.1 == 6 {
            if let MoveKind::Simple = check_move(board, double_move) {
                movements.push(double_move);
            }
        }
    }

    if let MoveKind::Capture = check_move(board, take_left) {
        movements.push(take_left);
    }

    if let MoveKind::Capture = check_move(board, take_right) {
        movements.push(take_right);
    }

    if let Some(position) = board.en_passant {
        if let MoveKind::Simple = check_move(board, take_left) {
            if position.0 == (origin.0 - 1) && position.1 == origin.1 {
                movements.push(take_left);
            }
        }

        if let MoveKind::Simple = check_move(board, take_right) {
            if position.0 == (origin.0 + 1) && position.1 == origin.1 {
                movements.push(take_right);
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

    let origin_piece = match board.get_piece(movement.origin) {
        Some(p) => p,
        None => return MoveKind::Invalid, // goes out if None
    };

    if let Some(target_piece) = board.get_piece(movement.target) {
        if origin_piece.color == target_piece.color {
            return MoveKind::Invalid;
        } else {
            return MoveKind::Capture;
        }
    } else {
        return MoveKind::Simple;
    }
}