mod board;
mod pieces;

use crate::board::Board;
use crate::pieces::Position;

fn main() {
    let board = Board::from_fen("r1q3kr/pbpppp1p/1pn3pQ/6N1/P3P3/R2P2P1/1PP2PBP/1N3RK1 w - - 3 13");
    println!("{:?}", board);
}
