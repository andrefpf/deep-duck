use std::fmt;
use crate::pieces::Position;
use crate::pieces::Piece;
use crate::pieces::Color;
use crate::movements::Movement;
use crate::pieces::PieceKind;
use crate::fen;

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

    pub fn ocuppied_squares(&self) -> Vec::<Piece> {
        let mut squares = Vec::<Piece>::with_capacity(32);

        for square in self.data {
            if let Some(piece) = square {
                squares.push(piece);
            }
        }
        
        squares
    }

    pub fn get_square(&self, pos: Position) -> Option<Piece> {
        let index = pos.0 + 8 * pos.1;
        self.data[index as usize]
    }
    
    pub fn set_square(&mut self, pos: Position, square: Option<Piece>) {
        let mut target_square = square;

        if let Some(mut piece) = square {
            piece.pos = pos;
            target_square = Some(piece);
        } 

        let index = pos.0 + 8 * pos.1;
        assert!(index >= 0);
        self.data[index as usize] = target_square;
    }

    pub fn move_piece(&mut self, origin: Position, target: Position) {
        let square = self.get_square(origin);
        self.move_counter = self.move_counter + 1;

        self.active_color = match self.active_color {
            Color::White => Color::Black,
            Color::Black => Color::White,
            Color::Yellow => panic!("Invalid active color"),
        };
    
        self.set_square(origin, None);
        self.set_square(target, square);
    }
    
    pub fn make_movement(&mut self, movement: Movement) {
        self.move_piece(movement.origin, movement.target);

        if let Some(kind) = movement.promotion {
            if let Some(mut piece) = self.get_square(movement.target) {
                piece.kind = kind;
                self.set_square(movement.target, Some(piece));
            }
        }
    }

    pub fn copy_and_move(&self, origin: Position, target: Position) -> Self {
        let mut board = self.clone();
        board.move_piece(origin, target);
        board
    }

    pub fn copy_movement(&self, movement: Movement) -> Self {
        let mut board = self.clone();
        board.make_movement(movement);
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
                let pos = Position(j, i);
                let square = self.get_square(pos);
                let representation = match square {
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