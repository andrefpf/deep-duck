mod board;
mod pieces;
mod movements;
mod engine;
mod fen;
mod evaluation;
mod cache;
mod cli;

use crate::cli::{App, Command};
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

// fn test() {
//     let start = Instant::now();

//     let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
//     search(&board, 6);

//     let duration = start.elapsed();
//     println!("Time elapsed: {:?}", duration);
// }

fn main() {
    let mut app = App::new();

    loop {
        print!(">> ");
        let input = get_input();
        match Command::from_str(input) {
            Command::Exit => break,
            Command::Empty => (),
            command => app.run(command),
        };
    }
}
