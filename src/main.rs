mod command_line_driver;
mod game_cell;
mod minesweeper;

use crate::command_line_driver::CommandLineDriver;
use crate::minesweeper::{GameGrid, MineSweeperGame};

fn main() {
    let g = GameGrid::init(4, 4);
    println!("{}", g);

    let mut command_driver = CommandLineDriver::new(g);
    command_driver.start();
}
