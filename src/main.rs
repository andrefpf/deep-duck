mod board;
mod pieces;
mod movements;

use crate::board::Board;
use crate::pieces::Position;
use crate::movements::piece_moves;

fn main() {
    for (i, j) in (0..8).zip(8..16) {
        println!("{} - {}", i, j);
    }

    // let board = Board::from_fen("r2q1r2/pbpp2k1/1pn2ppp/3N4/5Q2/1R1P1NP1/1PP2PBP/5RK1 w - - 0 20");
    // // let board = Board::arranged();
    // println!("{:?}", board);

    // let valid_movements = piece_moves(&board, Position(1, 2));
    // println!("Movimentos válidos: {}", valid_movements.len());
}
