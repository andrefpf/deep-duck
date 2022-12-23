use std::fmt;

use crate::pieces::PieceKind;
use crate::pieces::Position;
use crate::pieces::Piece;
use crate::pieces::Color;
use crate::movements::Movement;

#[derive(Clone)]
pub struct Castle {
    short: bool,
    long: bool,
}

#[derive(Clone)]
pub struct Board {
    data: [Option<Piece>; 64],
    pub last_move: Option<Movement>,
    pub en_passant: bool,
    pub active_piece: Color,
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
            last_move: None,
            en_passant: false,
            active_piece: Color::White,
            white_castle: Castle::new(),
            black_castle: Castle::new(),
        }
    }

    pub fn from_fen(notation: &str) -> Self {
        let mut board = Board::new();
        let mut x: i32 = 0;
        let mut y: i32 = 0;

        for c in notation.chars() {
            match c {
                ' ' => break,

                '/' => {
                    x = 0;
                    y = y + 1;
                },

                '1'..='8' => {
                    let int_c: i32 = c as i32 - 0x30;
                    x = x + int_c;
                },
                
                _ => {
                    board.set_piece(Position(x, 7-y), Some(Piece::from_fen(c)));
                    x = x + 1;
                }
            }
        }

        board
    }
    
    pub fn arranged() -> Self {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        board
    }

    pub fn get_piece(&self, pos: Position) -> Option<Piece> {
        let index = pos.0 + 8 * pos.1;
        assert!(index >= 0);
        self.data[index as usize]
    }
    
    fn set_piece(&mut self, pos: Position, piece: Option<Piece>) {
        let index = pos.0 + 8 * pos.1;
        assert!(index >= 0);
        self.data[index as usize] = piece;
    }
    
    pub fn move_piece(&self, origin: Position, target: Position) -> Self {
        let mut board = (*self).clone();
        let piece = board.get_piece(origin.clone());

        match piece {
            Some(Piece{color:Color::White, kind:PieceKind::Pawn}) => {
                board.last_move = Some(Movement{origin, target});
                board.active_piece = Color::Black;
                if origin.1 == 1 && target.1 == 3 {
                    board.en_passant = true;
                }
            },
            
            Some(Piece{color:Color::Black, kind:PieceKind::Pawn}) => {
                board.last_move = Some(Movement{origin, target});
                board.active_piece = Color::White;
                if origin.1 == 6 && target.1 == 4 {
                    board.en_passant = true;
                } 
            },

            _ => return board,
        }

        board.set_piece(origin, None);
        board.set_piece(target, piece);
        board
    }
    
    pub fn to_fen(&self) -> String {
        let mut notation = self.pieces_fen();
        
        notation.push(' ');
        match self.active_piece {
            Color::White => notation.push('w'),
            Color::Black => notation.push('b'),
        }
        notation.push(' ');
        notation.push_str(&self.castle_fen());
        notation.push(' ');
        notation.push_str(&self.en_passant_fen());
        notation.push_str(" 0 1");
        
        notation
    }

    fn pieces_fen(&self) -> String {
        let mut notation = String::new();
        let mut counter;

        for i in (0..8).rev() {
            counter = 0;
            for j in 0..8 {
                match self.get_piece(Position(j, i)) {
                    Some(piece) => {
                        if counter > 0 {
                            notation.push_str(&format!("{}", counter));
                            counter = 0;
                        }
                        notation.push(piece.fen_repr());
                    },
                    None => counter = counter + 1,
                }
            }
            if counter > 0 {
                notation.push_str(&format!("{}", counter));
            }
            notation.push('/');
        }
        notation.pop(); // remove last slash 

        notation
    }

    fn castle_fen(&self) -> String {
        let mut notation = String::new();
        let no_castle = !(self.white_castle.short || self.white_castle.long || self.black_castle.short || self.black_castle.long);

        if no_castle {
            notation.push('-');
            return notation;
        }

        if self.white_castle.short {
            notation.push('K')
        }
        if self.white_castle.long {
            notation.push('Q')
        }
        if self.black_castle.short {
            notation.push('k')
        }
        if self.black_castle.short {
            notation.push('q')
        }

        notation
    }

    fn en_passant_fen(&self) -> String {
        if !self.en_passant {
            return String::from("-");
        }

        if let Some(movement) = self.last_move {
            let indexes = "abcdefgh";
            if let Some(char_index) = indexes.chars().nth(movement.target.0 as usize) {
                return String::from(format!("{}{}", char_index, movement.target.1 + 1));
            }
        }

        String::from("-")
    }    
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::with_capacity(64);

        for i in (0..8).rev() {
            string.push_str(&format!("{} ", i + 1));

            for j in 0..8 {
                let square = self.get_piece(Position(j, i));
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