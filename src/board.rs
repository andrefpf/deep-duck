use std::fmt;
use crate::pieces::Position;
use crate::pieces::Piece;
use crate::pieces::Color;
use crate::movements::Movement;
use crate::fen;

pub struct Square {
    pub pos: Position,
    pub piece: Option<Piece>,
}

#[derive(Clone)]
pub struct Castle {
    pub short: bool,
    pub long: bool,
}

#[derive(Clone)]
pub struct Board {
    data: [Option<Piece>; 64],
    pub move_counter: usize,
    pub last_move: Option<Movement>,
    pub en_passant: bool,
    pub active_color: Color,
    pub white_castle: Castle,
    pub black_castle: Castle,
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
    pub fn new() -> Self {
        Board {
            data: [None; 64],
            move_counter: 0,
            last_move: None,
            en_passant: false,
            active_color: Color::White,
            white_castle: Castle::new(),
            black_castle: Castle::new(),
        }
    }

    pub fn from_fen(notation: &str) -> Self {
        fen::fen_to_board(notation)
    }
    
    pub fn arranged() -> Self {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        board
    }

    pub fn ocuppied_squares(&self) -> Vec::<Square> {
        let mut squares = Vec::<Square>::with_capacity(32);

        for i in 0..8 {
            for j in 0..8 {
                let square = self.get_square(i, j);
                if let Some(_) = square.piece {
                    squares.push(square);
                }
            }
        }        
        
        squares
    }

    pub fn get_square(&self, x: usize, y: usize) -> Square {
        let index = x + 8 * y;
        Square {
            pos: Position(x as i32, y as i32),
            piece:self.data[index as usize],
        }
    }
    
    pub fn set_square(&mut self, square: Square) {
        let index = square.pos.0 + 8 * square.pos.1;
        assert!(index >= 0);
        self.data[index as usize] = square.piece;
    }
    
    pub fn make_move(&mut self, origin: Position, target: Position) {
        let origin_square = self.get_square(origin.0 as usize, origin.1 as usize);
        self.move_counter = self.move_counter + 1;

        self.active_color = match self.active_color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    
        self.set_square(Square{pos:origin, piece:None});
        self.set_square(Square{pos:target, piece:origin_square.piece});
    }

    pub fn copy_and_move(&self, origin: Position, target: Position) -> Self {
        let mut board = self.clone();
        board.make_move(origin, target);
        board
    }
    
    pub fn to_fen(&self) -> String {
        fen::board_to_fen(self)
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::with_capacity(64);

        for i in (0..8).rev() {
            string.push_str(&format!("{} ", i + 1));

            for j in 0..8 {
                let square = self.get_square(j, i);
                let representation = match square.piece {
                    Some(piece) => piece.utf8_repr(),
                    None => ' ',
                };

                string.push(representation);
                string.push(' ');
            }
            string.push('\n');
        }
        string.push_str("  A B C D E F G H");

        write!(f, "{}", &string)
    }
}