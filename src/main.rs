mod board;
mod pieces;
mod movements;

use crate::board::Board;
use crate::pieces::Position;
use crate::movements::Movement;

fn main() {
    // let board = Board::from_fen("r2q1r2/pbpp2k1/1pn2ppp/3N4/5Q2/1Q1P1NP1/1PP2PBP/5RK1 w - - 0 20");
    let board = Board::arranged();
    let board = board.move_piece(Position(3, 1), Position(3, 3));
    let valid_movements = Movement::piece_moves(&board, Position(0, 1));
    
    println!("{:?}", board);
    println!("");
    println!("Movimentos válidos: {}", valid_movements.len());
    println!("FEN diagram: {}", board.to_fen());
}
