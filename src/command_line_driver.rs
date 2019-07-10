use crate::minesweeper::{MineSweeperGame, Game};
use std::io;
use std::io::{BufReader, BufRead, ErrorKind};
use crate::grid_cell::{GridCell, CellState, CellMarker};
use std::num::ParseIntError;

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

impl CommandLineDriver<Game<GridCell>> {

    pub fn new(game: Game<GridCell>) -> Self {
        CommandLineDriver { game }
    }

    pub fn start(&mut self) {
        loop {
            match CommandLineDriver::read_line() {
                Ok(command_str) => {
                    match CommandLineDriver::parse_command(command_str.as_str()) {
                        Ok(Command::Quit) => {
                            break
                        },
                        Ok(Command::Debug) => {
                            println!("{:?}", &self.game);
                        },
                        Ok(Command::New(r, c)) => {
                            self.game = Game::init(r, c);
                        },
                        Ok(Command::Flag(r, c)) => {
                            self.game.toggle_mark(r, c, CellMarker::Flagged)
                        },
                        Ok(Command::Question(r, c)) => {
                            self.game.toggle_mark(r, c, CellMarker::Questioned)
                        },
                        Ok(Command::Reveal(r, c)) => {
                            self.game.reveal_cell(r, c);
                        },
                        Err(e) => {
                            println!("{}", &e);
                        }
                    }
                },
                Err(e) => {
                    println!("{}", e);
                    break
                }
            }
            if self.game.is_game_over() {
                println!("you hit a mine!");
                println!("{:?}", self.game);
                break
            }
            if self.game.is_game_won() {
                println!("you win!!");
                println!("{:?}", self.game);
                break
            }
            println!("{}", self.game);
        }
    }

    pub fn read_line() -> io::Result<String> {
        print!("make a move: \n");
        let mut input = String::new();
        let mut br = BufReader::new( io::stdin() );
        let res = br.read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    pub fn parse_command(command: &str) -> Result<Command, io::Error> {
        fn parse_row_col(rc: &[&str]) -> Result<(usize, usize), io::Error> {
            let r = rc[0].parse::<usize>().map_err(|e| io::Error::new(ErrorKind::InvalidInput, format!("invalid row {}", rc[0]) ))?;
            let c = rc[1].parse::<usize>().map_err(|e| io::Error::new(ErrorKind::InvalidInput, format!("invalid col {}", rc[1]) ))?;
            Ok( (r, c) )
        }
        let cs = command.split_whitespace().collect::<Vec<&str>>();
        match cs[0] {
            "quit" => Ok(Command::Quit),
            "debug" => Ok(Command::Debug),
            "n" if cs.len() == 3 => {
                let (r, c) = parse_row_col( &cs[1..3] )?;
                Ok(Command::New(r, c))
            },
            "r" if cs.len() == 3  => {
                let (r,c) = parse_row_col( &cs[1..3] )?;
                Ok(Command::Reveal(r, c))
            },
            "f" if cs.len() == 3 => {
                let (r, c) = parse_row_col( &cs[1..3] )?;
                Ok(Command::Flag(r, c))
            },
            "q" if cs.len() == 3 => {
                let (r, c) = parse_row_col( &cs[1..3] )?;
                Ok(Command::Question(r, c))
            },
            _ => Err( io::Error::new( ErrorKind::InvalidInput, format!("invalid command {}", command)) ),
        }
    }
}