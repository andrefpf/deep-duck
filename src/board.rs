use std::fmt;

use crate::pieces::Piece;
use crate::pieces::Color;
use crate::pieces::Position;

#[derive(Clone)]
struct Castle {
    short: bool,
    long: bool,
}

#[derive(Clone)]
pub struct Board {
    data: [Option<Piece>; 64],
    white_castle: Castle,
    black_castle: Castle,
}

impl Castle {
    fn new() -> Self {
        Castle {
            short: true,
            long: true,
        }
    }
}

impl Board {
    fn new() -> Self {
        Board {
            data: [None; 64],
            white_castle: Castle::new(),
            black_castle: Castle::new(),
        }
    }

    pub fn arranged() -> Self {
        let mut board = Board::new();
        
        board.set_piece(Position{x:0, y:0}, Some(Piece::Rook(Color::White)));
        board.set_piece(Position{x:1, y:0}, Some(Piece::Knight(Color::White)));
        board.set_piece(Position{x:2, y:0}, Some(Piece::Bishop(Color::White)));
        board.set_piece(Position{x:3, y:0}, Some(Piece::Queen(Color::White)));
        board.set_piece(Position{x:4, y:0}, Some(Piece::King(Color::White)));
        board.set_piece(Position{x:5, y:0}, Some(Piece::Bishop(Color::White)));
        board.set_piece(Position{x:6, y:0}, Some(Piece::Knight(Color::White)));
        board.set_piece(Position{x:7, y:0}, Some(Piece::Rook(Color::White)));
        
        for i in 0..8 {
            board.set_piece(Position{x:i, y:1}, Some(Piece::Pawn(Color::White)));
            board.set_piece(Position{x:i, y:6}, Some(Piece::Pawn(Color::Black)));
        }

        board.set_piece(Position{x:0, y:7}, Some(Piece::Rook(Color::Black)));
        board.set_piece(Position{x:1, y:7}, Some(Piece::Knight(Color::Black)));
        board.set_piece(Position{x:2, y:7}, Some(Piece::Bishop(Color::Black)));
        board.set_piece(Position{x:3, y:7}, Some(Piece::Queen(Color::Black)));
        board.set_piece(Position{x:4, y:7}, Some(Piece::King(Color::Black)));
        board.set_piece(Position{x:5, y:7}, Some(Piece::Bishop(Color::Black)));
        board.set_piece(Position{x:6, y:7}, Some(Piece::Knight(Color::Black)));
        board.set_piece(Position{x:7, y:7}, Some(Piece::Rook(Color::Black)));

        board
    }

    fn get_piece(&self, pos: Position) -> Option<Piece> {
        self.data[pos.x + 8 * pos.y]
    }
    
    fn set_piece(&mut self, pos: Position, piece: Option<Piece>) {
        self.data[pos.x + 8 * pos.y] = piece;
    }

    pub fn move_piece(&self, origin: Position, target: Position) -> Self {
        let mut board = (*self).clone();
        let piece = board.get_piece(origin.clone());
        board.set_piece(origin, None);
        board.set_piece(target, piece);
        board
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::with_capacity(64);

        for i in (0..8).rev() {
            for j in 0..8 {
                let piece = self.get_piece(Position{x:j, y:i});
                let representation = Piece::repr(piece);
                string.push(representation);
                string.push(' ');
            }
            string.push('\n');
        }

        write!(f, "{}", &string)
    }
}