struct Movement {
    origin: Position,
    target: Position,
}

fn pieceMoves(board: &Board, position: Position) {

}

fn rookMoves(board: &Board, origin: Position) {
    let mut moves = [];
    let piece = board.get_piece(origin);

    for i in (origin.x+1)..8 {
        let target = Position{x:i, y:origin.y}
        match board.get_piece(target) {
            Some(Piece(color)) => {
                if color == piece.0 {
                    moves.push(Movement{origin, target})
                }
                break;
            },
            None => moves.push(Movement{origin, target}),
        }
    }

    for i in 0..(origin.x) {
        let target = Position{x:i, y:origin.y};
        match board.get_piece(target) {
            Some(Piece(color)) => {
                if color == piece.0 {
                    moves.push(Movement{origin, target})
                }
                break;
            },
            None => moves.push(Movement{origin, target}),
        }
    }

    for i in 0..8 {
        let target = Position{x:i, y:origin.y};
        let piece = board.get_piece(target);

        match board.get_piece(target) {
            Some(Piece(color)) => {
                if color == piece.0 {
                    moves.push(Movement{origin, target})
                }
                break;
            },
            None => moves.push(Movement{origin, target}),
        }
    }

    for i in 0..8 {
        let target = Position{x:i, y:origin.y}
        match board.get_piece(target) {
            Some(Piece(color)) => {
                if color == piece.0 {
                    moves.push(Movement{origin, target})
                }
                break;
            },
            None => moves.push(Movement{origin, target}),
        }
    }
}