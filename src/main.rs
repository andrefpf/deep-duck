mod board;
mod pieces;
mod movements;
mod engine;
mod fen;

use crate::board::Board;
use crate::engine::search;
use std::time::Instant;


fn example() {
    let mut board = Board::from_fen("r3k2r/p1ppqpb1/Bn2pnp1/3PN3/1p2P3/2N4p/PPPB1PPP/R3K2R b - - 0 1");
    println!("{:?} \n", board);
    
    let best_move = search(&board, 6);
    if let Some(best_move) = best_move {
        board.make_move(best_move.origin, best_move.target);
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
