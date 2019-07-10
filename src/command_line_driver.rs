use crate::minesweeper::{MineSweeperGame, Game};
use std::io;
use std::io::{BufReader, BufRead, ErrorKind};
use crate::grid_cell::{GridCell, CellMarker};

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

    /// starts the game and awaits input from stdin
    pub fn start(&mut self) {
        loop {
            match CommandLineDriver::read_line() {
                Ok(command_str) => {
                    match self.parse_command_line(command_str.as_str()) {
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
            if self.game.is_game_lost() {
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

    fn read_line() -> io::Result<String> {
        print!("make a move: \n");
        let mut input = String::new();
        let mut br = BufReader::new( io::stdin() );
        br.read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    fn parse_int(&self, s: &str) -> Result<usize, io::Error> {
        let idx = s.parse::<usize>()
            .map_err(|_e| io::Error::new(ErrorKind::InvalidInput, format!("invalid index given {}", s) ))?;
        Ok( idx )
    }

    fn index_bounds_check(idx: usize, max_idx: usize) -> Result<bool, io::Error> {
        if (0..max_idx).contains(&idx) {
            Ok( true )
        } else {
            Err( io::Error::new(ErrorKind::InvalidInput, format!("the index {} is out of the range 0..{}", idx, max_idx) ) )
        }
    }

    /// parses a minesweeper move containing a command plus a row and col index
    /// # Examples
    /// * "r 0 1" to reveal the cell at row 0 col 1
    /// * "f 1 2" to place a flag at row 1 col 2
    /// * "q 2 3" to place a question at row 2 col 3
    fn parse_move(&self, mov: &str, r_str: &str, c_str: &str ) -> Result<Command, io::Error> {
        let r = self.parse_int( r_str )?;
        let c = self.parse_int( c_str )?;
        CommandLineDriver::index_bounds_check(r, self.game.dimensions().0 )?;
        CommandLineDriver::index_bounds_check(c, self.game.dimensions().1 )?;
        match mov {
            "r" => Ok(Command::Reveal(r, c)),
            "f" => Ok(Command::Flag(r, c)),
            "q" => Ok(Command::Question(r, c)),
            _ => Err( io::Error::new( ErrorKind::InvalidInput, format!("invalid command {}", mov)) ),
        }
    }

    /// parses the entered command string
    fn parse_command_line(&self, command_str: &str) -> Result<Command, io::Error> {
        let toks = command_str.split_whitespace().collect::<Vec<&str>>();
        match toks[0] {
            "quit" => Ok(Command::Quit),
            "debug" => Ok(Command::Debug),
            "n" if toks.len() == 3 => {
                let r = self.parse_int( &toks[1] )?;
                let c = self.parse_int( &toks[2] )?;
                Ok(Command::New(r, c))
            },
            "r" | "f" | "q" if toks.len() == 3  => {
                self.parse_move( toks[0], toks[1], toks[2] )
            },
            _ => Err( io::Error::new( ErrorKind::InvalidInput, format!("invalid command {}", command_str)) ),
        }
    }
}