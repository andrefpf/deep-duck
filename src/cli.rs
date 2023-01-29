use crate::board::Board;
use crate::pieces::Position;
use crate::pieces::Color;
use crate::movements::Movement;
use crate::engine::search;
use crate::engine::evaluate;
use crate::evaluation::piece_value;
use crate::pieces::PieceKind;
use colored::Colorize;

#[derive(Debug)]
pub enum Command {
    Help,
    Board,
    Rearange,
    Evaluate,
    Sugest,
    Play,
    Fen(String),
    Depth(usize),
    Move(String),
    Analyze(String),
    Exit,
    Clear,
    Empty,
    Invalid,
}

pub struct App {
    board: Board,
    depth: usize,
}

const HELP_MESSAGE: &str = 
"                  __
    DEEP DUCK   <(o )___
versão: 0.2.0    (     /
         2023     `---'   
                

These are the avaliable commands:

    help            Shows this help message
    board           Shown the current board
    exit            Exits the engine

    rearange        Rearange the board to the initial position
    fen [notation]  Loads loads the board acording to the given FEN notation 
    depth [number]  Sets the maximum depth to evaluate

    evaluate        Evaluates the position and shows a pontuation
    sugest          The computer sugests the best movement
    play            The computer plays the best movement in the current board
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
            Command::Rearange => self.rearange(),
            Command::Evaluate => self.show_evaluation(),
            Command::Sugest => self.sugest_movement(),
            Command::Play => self.computer_move(),
            Command::Fen(fen) => self.load_board(&fen),
            Command::Move(coords) => self.try_movement(&coords),
            Command::Analyze(coords) => self.analyze_movement(&coords),
            Command::Depth(depth) => self.change_depth(depth),
            Command::Clear => App::clear_terminal(),
            Command::Invalid => App::invalid(),
            Command::Exit | Command::Empty => (),
        }
    }

    fn decode_positions(coords: &str) -> Option<(Position, Position, Position)> {
        let mut splited = coords.split(' ');

        let origin = match splited.next() {
            Some(coord) => Position::from_str(coord),
            None => None
        };

        let target = match splited.next() {
            Some(coord) => Position::from_str(coord),
            None => None
        };

        let duck = match splited.next() {
            Some(coord) => Position::from_str(coord),
            None => None
        };

        if let (Some(origin), Some(target), Some(duck)) = (origin, target, duck) {
            Some((origin, target, duck))
        } else {
            None
        }
    }

    fn try_movement(&mut self, coords: &str) {
        let decoded = App::decode_positions(coords);

        if decoded.is_none() {
            App::invalid();
            return;
        }

        let (origin, target, duck) = decoded.unwrap();

        if let Some(movement) = Movement::try_movement(&self.board, origin, target, duck) {
            self.board.make_movement(movement);
            println!("{:?}", self.board);    
            println!("You moved: {:?} to {:?} and duck to {:?}", movement.origin, movement.target, movement.duck_target);
        } else {
            App::invalid_movement();
        }
    }

    fn analyze_movement(&mut self, coords: &str) {
        let decoded = App::decode_positions(coords);

        if decoded.is_none() {
            App::invalid();
            return;
        }

        let (origin, target, duck) = decoded.unwrap();

        if let Some(movement) = Movement::try_movement(&self.board, origin, target, duck) {
            let tmp_board = self.board.copy_movement(movement);
            let done = -evaluate(&tmp_board, self.depth-1).score;
            let expected = evaluate(&self.board, self.depth).score;
            App::compare_scores(done, expected)
        } else {
            App::invalid_movement();
        }
    }

    fn compare_scores(done: i32, expected: i32) {
        let diff = expected - done;

        if (expected >= piece_value(PieceKind::King))
            && (done < piece_value(PieceKind::King)
            && (done > 250))
        {
            println!("{} Missed Win", ":(".yellow().bold());
            return;
        }

        match diff {
            -20 ..= 20 => println!("{} Excelent", ":D".green().bold()),
            -50 ..= 50 => println!("{} :) Good", ":)".green()),
            -250 ..= 250 => println!("{} Inaccuracy", "!?".yellow()),
            _ => println!("{} BLUNDER", "??".red().bold()),
        }
    }

    fn invalid_movement() {
        println!("This is not a valid duck chess movement.")
    }

    fn rearange(&mut self) {
        self.board = Board::arranged();
    }

    fn print_help(&self) {
        println!("{}", HELP_MESSAGE);
    }
    
    fn print_board(&self) {
        println!("{:?}", self.board)
    }

    fn show_evaluation(&self) {
        let evaluation = evaluate(&self.board, self.depth);

        let score = match self.board.active_color {
            Color::White => evaluation.score,
            Color::Black => -evaluation.score,
            _ => 0,
        };

        // this is cringe but very convenient =)
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

        if score >= 1_000_000 {
            println!("White has mate in {}", evaluation.depth)
        }
        else if score <= -1_000_000 {
            println!("Black has mate in {}", evaluation.depth)
        }
        else {
            println!("Points: {}", score/100)
        }
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

    fn load_board(&mut self, fen: &str) {
        self.board = Board::from_fen(fen);
        println!("{:?}", self.board);
    }

    fn change_depth(&mut self, depth: usize) {
        if depth > 6 {
            println!("Be carefull, this may take an eternity to run.")
        }
        self.depth = depth
    }

    fn clear_terminal() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    fn invalid() {
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
            "clear" => Command::Clear,
            "help" => Command::Help,
            "board" => Command::Board,
            "rearange" => Command::Rearange,
            "evaluate" => Command::Evaluate,
            "sugest" => Command::Sugest,
            "play" => Command::Play,
            "fen" => Command::Fen(val.to_string()),
            "move" => Command::Move(val.to_string()),
            "analyze" => Command::Analyze(val.to_string()),
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
