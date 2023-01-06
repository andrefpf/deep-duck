mod board;
mod pieces;
mod movements;
mod engine;
mod fen;

use crate::board::Board;
use crate::engine::search;
use std::time::Instant;


fn example() {
    let mut board = Board::from_fen("k7/8/8/2K5/8/8/8/8 w - - 0 1");
    println!("{:?} \n", board);
    
    let best_move = search(&board, 3);
    if let Some(best_move) = best_move {
        board.make_duck_movement(best_move);
        println!("{:?}", best_move);
        println!("{:?}", board);
    }
        
    println!("{}", board.to_fen());
}

fn main() {
    let start = Instant::now();
    example();
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}
