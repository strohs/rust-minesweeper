use crate::game_cell::{CellMarker, GameCell};
use crate::minesweeper::{GameGrid, MineSweeperGame};
use std::io;
use std::io::{BufRead, BufReader, ErrorKind};

/// enables a user to play a Mine Sweeper Game via the command line (stdin)
///
/// The user will enter commands using a space separated string in one of the following formats:
/// * to create a new game with 5 rows and 5 columns: `n 5 5`
/// * to reveal the square at row 0 column 1: `r 0 1`
/// * to flag a square at row 2 column 4: `f 2 4`
/// * to question a square at row 1 column 3: `q 1 3`

pub struct CommandLineDriver<T: MineSweeperGame> {
    pub game: T,
}

#[derive(Debug)]
pub enum Command {
    Quit,
    Debug,
    New(usize, usize),
    Reveal(usize, usize),
    Flag(usize, usize),
    Question(usize, usize),
}

impl CommandLineDriver<GameGrid<GameCell>> {
    pub fn new(game: GameGrid<GameCell>) -> Self {
        CommandLineDriver { game }
    }

    /// starts a minesweeper game and waits for input from stdin
    pub fn start(&mut self) {
        loop {
            match CommandLineDriver::read_line() {
                Ok(command_str) => match self.parse_command_line(command_str.as_str()) {
                    Ok(Command::Quit) => break,
                    Ok(Command::Debug) => {
                        println!("{:?}", &self.game);
                    }
                    Ok(Command::New(r, c)) => {
                        self.game = GameGrid::init(r, c);
                    }
                    Ok(Command::Flag(r, c)) => self.game.toggle_mark(r, c, CellMarker::Flagged),
                    Ok(Command::Question(r, c)) => {
                        self.game.toggle_mark(r, c, CellMarker::Questioned)
                    }
                    Ok(Command::Reveal(r, c)) => {
                        self.game.reveal_cell(r, c);
                    }
                    Err(e) => {
                        println!("{}", &e);
                    }
                },
                Err(e) => {
                    println!("{}", e);
                    break;
                }
            }
            if self.game.is_game_lost() {
                println!("you hit a mine!");
                println!("{:?}", self.game);
                break;
            }
            if self.game.is_game_won() {
                println!("you win!!");
                println!("{:?}", self.game);
                break;
            }
            println!("{}", self.game);
        }
    }

    fn read_line() -> io::Result<String> {
        print!("make a move: \n");
        let mut input = String::new();
        let mut br = BufReader::new(io::stdin());
        br.read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    fn parse_int(&self, s: &str) -> Result<usize, io::Error> {
        let idx = s.parse::<usize>().map_err(|_e| {
            io::Error::new(
                ErrorKind::InvalidInput,
                format!("invalid index given {}", s),
            )
        })?;
        Ok(idx)
    }

    fn check_index_bounds(idx: usize, max_idx: usize) -> Result<bool, io::Error> {
        if (0..max_idx).contains(&idx) {
            Ok(true)
        } else {
            Err(io::Error::new(
                ErrorKind::InvalidInput,
                format!("the index {} is out of the range 0..{}", idx, max_idx),
            ))
        }
    }

    /// maps a minesweeper "move" into a minesweeper `Command` enum
    /// # Examples
    /// * "r 0 1" to reveal the cell at row 0 col 1
    /// * "f 1 2" to place a flag at row 1 col 2
    /// * "q 2 3" to place a question at row 2 col 3
    fn map_move(&self, command: &str, row: &str, col: &str) -> Result<Command, io::Error> {
        let r = self.parse_int(row)?;
        let c = self.parse_int(col)?;
        CommandLineDriver::check_index_bounds(r, self.game.dimensions().0)?;
        CommandLineDriver::check_index_bounds(c, self.game.dimensions().1)?;
        match command {
            "r" => Ok(Command::Reveal(r, c)),
            "f" => Ok(Command::Flag(r, c)),
            "q" => Ok(Command::Question(r, c)),
            _ => Err(io::Error::new(
                ErrorKind::InvalidInput,
                format!("invalid command {}", command),
            )),
        }
    }

    /// parses the entered command string
    fn parse_command_line(&self, command_str: &str) -> Result<Command, io::Error> {
        let toks = command_str.split_whitespace().collect::<Vec<&str>>();
        match toks[0] {
            "quit" => Ok(Command::Quit),
            "debug" => Ok(Command::Debug),
            "n" if toks.len() == 3 => {
                let r = self.parse_int(&toks[1])?;
                let c = self.parse_int(&toks[2])?;
                Ok(Command::New(r, c))
            }
            "r" | "f" | "q" if toks.len() == 3 => self.map_move(toks[0], toks[1], toks[2]),
            _ => Err(io::Error::new(
                ErrorKind::InvalidInput,
                format!("invalid command {}", command_str),
            )),
        }
    }
}
