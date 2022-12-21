mod board;
mod pieces;

use crate::board::Board;
use crate::pieces::Position;

fn main() {
    // let board = Board::from_fen("r2q1r2/pbpp2k1/1pn2ppp/3N4/P4Q2/R2P1NP1/1PP2PBP/5RK1 w - - 0 20");
    let board = Board::arranged();
    println!("{:?}", board);
}
