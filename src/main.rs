mod minesweeper;
mod cell;

use crate::cell::{Cell,MineSweeperCell};
use crate::minesweeper::{Game, MineSweeperGame};


fn main() {
    let mut g = Game::new(5, 5);
    g.randomize_mine_locations();
    println!("{:?}", g);
    let acs = g.adjacent_cells(2,2);
    println!("{:?}", acs);
}
