mod minesweeper;
mod cell;

use crate::cell::{Cell,MineSweeperCell};
use crate::minesweeper::{Game, MineSweeperGame};


fn main() {
    let mut g = Game::new(4, 4);
    g.init();
    println!("{:?}", g);

    let locs = g.mine_locations();
    println!("{:?}", locs);
}
