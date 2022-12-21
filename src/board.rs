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
    active_piece: Color,
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
    pub fn new() -> Self {
        Board {
            data: [None; 64],
            active_piece: Color::White,
            white_castle: Castle::new(),
            black_castle: Castle::new(),
        }
    }

    pub fn from_fen(notation: &str) -> Self {
        let mut board = Board::new();
        let mut x = 0;
        let mut y = 0;

        for c in notation.chars() {
            match c {
                ' ' => break,

                '/' => {
                    x = 0;
                    y = y + 1;
                },

                '1'..='8' => {
                    let int_c = c as usize - 0x30;
                    x = x + int_c;
                },
                
                _ => {
                    board.set_piece(Position{x, y:(7-y)}, Piece::from_fen(c));
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

    fn pieces_fen(&self) -> String {
        let mut notation = String::new();
        let mut counter;

        for i in (0..8).rev() {
            counter = 0;
            for j in 0..8 {
                match self.get_piece(Position{x:j, y:i}) {
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
                counter = 0;
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
        notation.push_str("- 0 1");
        
        notation
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
            string.push_str(&format!("{} ", i + 1));

            for j in 0..8 {

                let square = self.get_piece(Position{x:j, y:i});
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