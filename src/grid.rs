use std::fmt;

use crate::cell::{Cell, MineSweeperCell, CellKind};

pub struct Grid<T: MineSweeperCell> {
    grid: Vec<Vec<T>>
}

//pub trait MineSweeperGrid {
//    fn dimensions(&self) -> (usize, usize) { (0,0) }
//    //fn set_mine_locations(&mut self, indices: Vec<(usize, usize)>);
//    //fn reveal_cell(&mut self, r: usize, c:usize) -> impl MineSweeperCell;
//    //fn mark_cell(&mut self, r:usize, c:usize) -> impl MineSweeperCell;
//}


impl<T> Grid<T>
    where T: MineSweeperCell {

    pub fn new(rows: usize, cols:usize) -> Grid<T> {
        let mut board= vec![];
        for r in 0..rows {
            let mut row = vec![];
            for c in 0..cols {
                row.push(Cell::new(CellKind::Empty));
            }
            board.push(row)
        }
        Grid {
            grid: board
        }
    }
}

//impl fmt::Display for Grid {
//
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        let mut buf = String::new();
//        for row in self.grid.iter() {
//            for cell in row.iter() {
//                buf.push_str( format!(" {}", cell).as_str() );
//            }
//            buf.push_str("\n")
//        }
//        write!(f,"{}", buf)
//    }
//}