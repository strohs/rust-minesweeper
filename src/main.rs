mod minesweeper;
mod grid_cell;

use crate::grid_cell::{GridCell, MineSweeperCell};
use crate::minesweeper::{Game, MineSweeperGame};


fn main() {
    let mut g = Game::init(4, 4);
    println!("{:?}", g);

    g.reveal_cell(0,0);
    println!("{}", g);
//    let locs = g.mine_locations();
//    println!("{:?}", locs);
}
