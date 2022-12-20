use std::fmt;

#[derive(Clone)]
struct Castle {
    short: bool,
    long: bool,
}

#[derive(Copy, Clone, Debug)]
enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug)]
enum Piece {
    Pawn(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Queen(Color),
    King(Color),
}

#[derive(Copy, Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Board {
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

    fn arranged() -> Self {
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

    fn move_piece(&self, origin: Position, target: Position) -> Self {
        let mut board = (*self).clone();
        let piece = board.get_piece(origin.clone());
        board.set_piece(origin, None);
        board.set_piece(target, piece);
        board
    }
}

impl Piece {
    fn repr(piece: Option<Self>) -> char {
        match piece {
            Some(Piece::Rook(Color::Black)) => '♖',
            Some(Piece::Knight(Color::Black)) => '♘',
            Some(Piece::Bishop(Color::Black)) => '♗',
            Some(Piece::Queen(Color::Black)) => '♕',
            Some(Piece::King(Color::Black)) => '♔',
            Some(Piece::Pawn(Color::Black)) => '♙',

            Some(Piece::Rook(Color::White)) => '♜',
            Some(Piece::Knight(Color::White)) => '♞',
            Some(Piece::Bishop(Color::White)) => '♝',
            Some(Piece::Queen(Color::White)) => '♛',
            Some(Piece::King(Color::White)) => '♚',
            Some(Piece::Pawn(Color::White)) => '♟',

            None => ' ',
        }
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

fn main() {
    let board = Board::arranged();

    let moved = board.move_piece(Position{x:0, y:0}, Position{x:4, y:4});
    println!("{:?}", moved);
}
