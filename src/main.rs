mod board;
mod pieces;
mod movements;
mod engine;

use crate::board::Board;
use crate::pieces::Position;
use crate::pieces::Color;
use crate::movements::Movement;
use crate::engine::search;
use crate::engine::evaluate;

fn main() {
    let mut board = Board::from_fen("4R3/8/5R2/2KN4/8/8/8/8 w - - 0 1");
    println!("{:?} \n", board);

    let mut color = Color::White;
    let best_move = search(&board, color, 2);

    if let Some(best_move) = best_move {
        board.make_move(best_move.origin, best_move.target);
        println!("{:?}", best_move);
        println!("{:?}", board);
    }

    println!("{}", board.to_fen());
}
