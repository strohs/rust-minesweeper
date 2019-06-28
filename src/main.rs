mod grid;
mod cell;

use crate::grid::{Grid};
use crate::cell::{Cell,MineSweeperCell};


fn main() {
    let mut g: Grid<MineSweeperCell> = Grid::new(5, 3);
    println!("done!");
}
