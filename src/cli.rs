use crate::board::Board;
use crate::pieces::Color;
use crate::engine::search;
use crate::engine::evaluate;

#[derive(Debug)]
pub enum Command {
    Help,
    Board,
    Restart,
    Evaluate,
    Sugest,
    Play,
    Fen(String),
    Depth(usize),
    Exit,
    Clear,
    Empty,
    Invalid,
}

pub struct App {
    board: Board,
    depth: usize,
}

const HELP_MESSAGE: &str = "
DEEP DUCK
versão: 0.1.0

These are the avaliable commands:

    exit \t\t Stop this
    help \t\t  Shows this help message
    board \t\t Shown the current board
    fen [fen notation] \t\t Loads another board
    depth [number] \t\t Updates the maximum depth
";

impl App {
    pub fn new() -> Self {
        App {
            board: Board::arranged(),
            depth: 6,
        }
    }

    pub fn run(&mut self, command: Command) {
        match command {
            Command::Help => self.print_help(),
            Command::Board => self.print_board(),
            Command::Restart => self.restart(),
            Command::Evaluate => self.show_evaluation(),
            Command::Sugest => self.sugest_movement(),
            Command::Play => self.computer_move(),
            Command::Fen(fen) => self.load_board(fen),
            Command::Depth(depth) => self.change_depth(depth),
            Command::Clear => self.clear_terminal(),
            Command::Invalid => self.invalid(),
            Command::Exit | Command::Empty => (),
        }
    }

    fn restart(&mut self) {
        let tmp = Self::new();
        self.depth = tmp.depth;
    }

    fn print_help(&self) {
        println!("{}", HELP_MESSAGE);
    }
    
    fn print_board(&self) {
        println!("{:?}", self.board)
    }

    fn show_evaluation(&self) {
        let score = match self.board.active_color {
            Color::White => evaluate(&self.board, self.depth),
            Color::Black => -evaluate(&self.board, self.depth),
            _ => 0,
        };

        // this is as cringe but very convenient =)
        let bar = match score {
            ..= -1_000_000         => "○○○○○○○○○○○○○○○○○○○○",
            (-999_999 ..= -1_000)  => "●○○○○○○○○○○○○○○○○○○○",
            (-999  ..= -300)       => "●●●●●○○○○○○○○○○○○○○○",
            (-299 ..= -100)        => "●●●●●●●●○○○○○○○○○○○○",
            (-99 ..= 99)           => "●●●●●●●●●●○○○○○○○○○○",
            (100  ..= 299)         => "●●●●●●●●●●●●○○○○○○○○",
            (300  ..= 999)         => "●●●●●●●●●●●●●●●●○○○○",
            (1_000  ..= 999_999)   => "●●●●●●●●●●●●●●●●●●●○",
            1_000_000 ..           => "●●●●●●●●●●●●●●●●●●●●",
        };

        match score {
            1_000_000.. => println!("White has a mate"),
            ..= -1_000_000 => println!("Black has a mate"),
            _ => println!("Centipawns: {}", score/100),
        };
        println!("{}", bar);
    }

    fn sugest_movement(&self) {
        let best_move = search(&self.board, self.depth);
        if let Some(movement) = best_move {
            println!("Move: {:?} to {:?} and duck to {:?}", movement.origin, movement.target, movement.duck_target);
        } else {
            println!("There are no movements for your position.");
        }
    }

    fn computer_move(&mut self) {
        let best_move = search(&self.board, self.depth);
        if let Some(movement) = best_move {
            self.board.make_movement(movement);
            println!("{:?}", self.board);
            println!("Computer moved: {:?} to {:?} and duck to {:?}", movement.origin, movement.target, movement.duck_target);
        } else {
            println!("There are no movements for this position.");
        }
    }

    fn load_board(&mut self, fen: String) {
        self.board = Board::from_fen(&fen);
        println!("{:?}", self.board);
    }

    fn change_depth(&mut self, depth: usize) {
        if depth > 6 {
            println!("Be carefull, this may take an eternity to run.")
        }
        self.depth = depth
    }

    fn clear_terminal(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    fn invalid(&self) {
        println!("Invalid command. Type help for more info.")
    }
}

impl Command {
    pub fn from_str(input: String) -> Self {
        let (key, val) = match input.trim().split_once(' ') {
            Some(val) => val,
            None => (input.trim(), ""),            
        };  
    
        match key {
            "" => Command::Empty,
            "exit" => Command::Exit,
            "help" => Command::Help,
            "board" => Command::Board,
            "restart" => Command::Restart,
            "evaluate" => Command::Evaluate,
            "sugest" => Command::Sugest,
            "play" => Command::Play,
            "fen" => Command::Fen(val.to_string()),
            "clear" => Command::Clear,
            "depth" => {
                if let Ok(number) = val.parse::<usize>() {
                    Command::Depth(number)
                } else {
                    Command::Invalid
                }
            },
            _ => Command::Invalid,
        }
    }
}
