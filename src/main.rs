mod board;
mod pieces;

use crate::board::Board;
use crate::pieces::Position;

fn main() {
    let board = Board::arranged();

    let moved = board.move_piece(Position{x:0, y:0}, Position{x:0, y:2});
    println!("{:?}", moved);
}
