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
    let mut board = Board::from_fen("5r1k/p5p1/1qb1p3/1p6/1PnPB3/1R6/2Q1KBPP/8 b - - 4 29");
    println!("{:?} \n", board);

    let mut color = Color::Black;

    
    use std::time::Instant;
    let now = Instant::now();

    let best_move = search(&board, color);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    













    if let Some(best_move) = best_move {
        board.make_move(best_move.origin, best_move.target);
        println!("{:?}", best_move);
        println!("{:?}", board);
    }

    println!("{}", board.to_fen());
}
