mod board;
mod pieces;
mod movements;
mod engine;
mod fen;
mod evaluation;

use crate::board::Board;
use crate::engine::search;
use crate::movements::perft;
use std::time::Instant;
use std::io::{stdin,stdout,Write};


fn get_input() -> String {
    let mut input = String::new();

    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Did not enter a correct string");

    if let Some('\n')=input.chars().next_back() {
        input.pop();
    }

    if let Some('\r')=input.chars().next_back() {
        input.pop();
    }

    input
}

fn cli() {
    loop {
        print!("Your FEN position: ");
        let fen = get_input();

        let start = Instant::now();
        let mut board = Board::from_fen(&fen);
        let best_move = search(&board, 5);
        let duration = start.elapsed();
        
        
        if let Some(movement) = best_move {
            board.make_movement(movement);
            println!("{:?}", board);
            println!("Move: {:?} to {:?} and duck to {:?}", movement.origin, movement.target, movement.duck_target);
            println!("Time elapsed: {:?}", duration);
        } else {
            println!("{:?}", board);
            println!("There are no movements for your position.");
        }
        println!();
    }
}

fn main() {
    // cli()
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ");

    dbg!(perft(&board, 3));
}
