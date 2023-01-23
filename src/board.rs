use std::fmt;
use crate::pieces::Position;
use crate::pieces::Piece;
use crate::pieces::Color;
use crate::movements::Movement;
use crate::pieces::PieceKind;
use crate::fen;
use std::iter::Flatten;
use std::slice::Iter;

#[derive(Clone)]
pub struct Castle {
    pub short: bool,
    pub long: bool,
}

#[derive(Clone)]
pub struct Board {
    data: [Option<Piece>; 64],
    pub duck: Option<Position>,
    pub move_counter: usize,
    pub active_color: Color,
}

impl Board {
    pub fn new() -> Self {
        Board {
            data: [None; 64],
            duck: None,
            move_counter: 0,
            active_color: Color::White,
        }
    }

    pub fn from_fen(notation: &str) -> Self {
        fen::fen_to_board(notation)
    }

    #[allow(dead_code)]
    pub fn to_fen(&self) -> String {
        fen::board_to_fen(self)
    }
    
    #[allow(dead_code)]
    pub fn arranged() -> Self {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn ocuppied_squares(&self) -> Flatten<Iter<Option<Piece>>> {
        self.data.iter().flatten()
    }

    pub fn get_square(&self, pos: Position) -> Option<Piece> {
        let index = pos.0 + 8 * pos.1;
        self.data[index as usize]
    }

    pub fn clear_square(&mut self, pos: Position) {
        if let Some(duck) = self.duck {
            if duck == pos {
                self.duck = None;
            }
        }

        let index = pos.0 + 8 * pos.1;
        assert!(index >= 0);
        self.data[index as usize] = None;
    }
    
    pub fn set_square(&mut self, piece: Piece) {
        if let PieceKind::Duck = piece.kind {
            self.duck = Some(piece.pos);
        }

        let index = piece.pos.0 + 8 * piece.pos.1;
        assert!(index >= 0);
        self.data[index as usize] = Some(piece);
    }

    pub fn drag_piece(&mut self, origin: Position, target: Position) {
        if let Some(mut square) = self.get_square(origin) {
            square.pos = target;
            self.move_counter += 1;
            self.set_square(square);
            self.clear_square(origin);
        }
    }

    pub fn make_movement(&mut self, movement: Movement) {
        self.drag_piece(movement.origin, movement.target);
        self.place_duck(Some(movement.duck_target));
        self.update_color();

        if let Some(kind) = movement.promotion {
            let mut piece = self.get_square(movement.target).unwrap();
            piece.kind = kind;
            self.set_square(piece);
        }
    }

    #[allow(dead_code)]
    pub fn unmake_movement(&mut self, movement: Movement) {
        self.update_color();
        self.place_duck(movement.duck_origin);
        self.drag_piece(movement.target, movement.origin);

        if let Some(captured) = movement.captured {
            self.set_square(captured);
            // let piece = Piece {pos:movement.target, kind:captured.kind, color:movement.color.invert()};
        }

        // if let Some(kind) = movement.promotion {
        //     if let Some(mut piece) = self.get_square(movement.target) {
        //         piece.kind = kind;
        //         self.set_square(piece);
        //     }
        // }
    }

    pub fn copy_movement(&self, movement: Movement) -> Self {
        let mut board = self.clone();
        board.make_movement(movement);
        board
    }

    pub fn place_duck(&mut self, position: Option<Position>) {
        if let Some(duck) = self.duck {
            self.clear_square(duck);
        }

        if let Some(pos) = position {
            let piece = Piece {pos, color:Color::Yellow, kind:PieceKind::Duck};
            self.set_square(piece);
        }
    }

    fn update_color(&mut self) {
        self.active_color = self.active_color.invert();
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