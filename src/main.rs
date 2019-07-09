mod minesweeper;
mod grid_cell;
mod command_line_adapter;

use crate::grid_cell::{GridCell, MineSweeperCell};
use crate::minesweeper::{Game, MineSweeperGame};
use crate::command_line_adapter::{CommandLineAdapter, Command};


fn main() {
    let mut g = Game::init(4, 4);
    println!("{:?}", g);

    let mut cla = CommandLineAdapter::new( g );

    loop {
        match CommandLineAdapter::read_line() {
            Ok(command_str) => {
                match CommandLineAdapter::parse_command(command_str.as_str()) {
                    Ok(Command::Quit) => {
                        break
                    },
                    Ok(Command::New(r, c)) => {
                        cla.game = Game::init(r, c);
                    },
                    Ok(Command::Flag(r, c)) => {
                        cla.game.flag_cell(r, c);
                    },
                    Ok(Command::Question(r, c)) => {
                        cla.game.question_cell(r, c);
                    },
                    Ok(Command::Reveal(r, c)) => {
                        cla.game.reveal_cell(r, c);
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
        if cla.game.is_game_over() {
            println!("you hit a mine!");
            println!("{:?}", cla.game);
            break
        }
        if cla.game.is_game_won() {
            println!("you win!!");
            println!("{:?}", cla.game);
            break
        }
        println!("{}", cla.game);
    }
}
