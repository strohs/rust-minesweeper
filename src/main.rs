mod minesweeper;
mod grid_cell;
mod command_line_driver;

use crate::minesweeper::{Game, MineSweeperGame};
use crate::command_line_driver::{CommandLineDriver};


fn main() {
    let g = Game::init(4, 4);
    println!("{}", g);

    let mut cla = CommandLineDriver::new( g );
    cla.start();

}
