mod board;
mod pieces;
mod movements;
mod engine;
mod fen;

use crate::board::Board;
use crate::engine::search;
use std::time::Instant;


fn example() {
    let mut board = Board::from_fen("2r1q3/pb4pk/1pn1p1Np/3pP3/3P4/2P5/P1Q3PP/5RK1 w - - 0 23");
    println!("{:?} \n", board);
    
    let best_move = search(&board, 5);
    if let Some(best_move) = best_move {
        board.make_movement(best_move);
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
