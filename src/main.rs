mod board;
mod pieces;
mod movements;
mod engine;
mod fen;
mod evaluation;

use crate::board::Board;
use crate::engine::search;
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
            println!("Move: {:?} to {:?} and duck to {:?}", movement.origin, movement.target, movement.duck);
            println!("Time elapsed: {:?}", duration);
        } else {
            println!("There are no movements for your position.");
        }
        println!();
    }
}

fn example() {
    let mut board = Board::from_fen("k7/8/8/2K5/8/8/8/8 w - - 0 1");
    println!("{:?} \n", board);
    
    let best_move = search(&board, 3);
    if let Some(best_move) = best_move {
        board.make_movement(best_move);
        println!("{:?}", best_move);
        println!("{:?}", board);
    }
        
    println!("{}", board.to_fen());
}

fn main() {
    cli()
}
