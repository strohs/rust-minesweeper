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
    cla.start();

}
