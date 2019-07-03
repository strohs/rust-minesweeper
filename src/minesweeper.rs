use std::fmt;
use rand::{thread_rng};
use rand::seq::{SliceRandom};

use crate::cell::{Cell, MineSweeperCell, CellKind};
use core::borrow::BorrowMut;

pub struct Game<T: MineSweeperCell> {
    grid: Vec<Vec<T>>,
    elapsed_time: u64,
}

pub trait MineSweeperGame {
    fn dimensions(&self) -> (usize, usize);
    fn total_mines(&self) -> usize;
    fn randomize_mine_locations(&mut self);
    fn cell_exists(&self, r: usize, c: usize) -> bool;
    fn adjacent_cells(&mut self, r: usize, c: usize) -> Vec<&mut Cell>;
//    fn reveal_cell(&mut self, r: usize, c:usize) -> impl MineSweeperCell;
//    fn mark_cell(&mut self, r:usize, c:usize) -> impl MineSweeperCell;
}

impl Game<Cell> {
    pub fn new(rows: usize, cols:usize) -> Game<Cell> {
        let mut grid = Vec::with_capacity(rows);
        for _r in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for _c in 0..cols {
                row.push(Cell::new(CellKind::Empty))
            }
            grid.push(row)
        }
        Game {
            grid,
            elapsed_time: 0,
        }
    }

}

impl MineSweeperGame for Game<Cell> {
    fn dimensions(&self) -> (usize, usize) {
        (self.grid.len(), self.grid[0].len())
    }

    /// computes the total mines that should be on a grid, using the grid's dimensions
    /// TOTAL_MINES = grid.rows * grid.columns * 0.15
    fn total_mines(&self) -> usize {
        let (r,c) = self.dimensions();
        ((r * c) as f32 * 0.15f32).round() as usize
    }

    /// generates random mine locations on the grid
    fn randomize_mine_locations(&mut self) {
        let (r,c) = self.dimensions();
        // build a vec of all grid indices in row major form and shuffle them
        let mut rng = thread_rng();
        let mut grid_indices: Vec<usize> = (0..(r * c)).map(|i| i).collect();
        grid_indices.shuffle(&mut rng);

        // set the mine locations in the grid
        for idx in grid_indices.iter().take(self.total_mines()) {
            let row = idx / c;
            let col = idx - ((idx / c) * c);
            self.grid[row][col] = Cell::new(CellKind::Mine );
        }
    }

    fn cell_exists(&self, r: usize, c: usize) -> bool {
        if let Some(row) = self.grid.get(r) {
            if let Some(cell) = row.get(c) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn adjacent_cells(&mut self, r: usize, c: usize) -> Vec<&mut Cell> {
        let mut cells: Vec<&mut Cell> = vec![];
        cells.push( &mut self.grid[0][0] );
        cells.push( &mut self.grid[1][1] );
        //if self.cell_exists(r-1,c) { cells.push( &mut self.grid[r-1][c] ) }
        //if self.cell_exists(r-1,c-1) { cells.push( &mut self.grid[r-1][c-1] ) }
//        if let Some(top) = self.get_cell(r-1, c) { cells.push(top); }
//        if let Some(top_left) = self.get_cell(r-1, c-1) { cells.push(top_left); }
//        if let Some(top_right) = self.get_cell(r+1, c+1) {cells.push(top_right); }
//        if let Some(right) = self.get_cell(r, c+1) { cells.push(right); }
//        if let Some(bot_right) = self.get_cell(r+1, c+1) { cells.push(bot_right); }
//        if let Some(bot) = self.get_cell(r+1, c) { cells.push(bot); }
//        if let Some(bot_left) = self.get_cell(r+1, c-1) { cells.push(bot_left); }
//        if let Some(left) = self.get_cell(r-1, c) { cells.push(left); }
        cells
    }
}

impl fmt::Display for Game<Cell> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for row in self.grid.iter() {
            for cell in row.iter() {
                buf.push_str( format!(" {}", cell).as_str() );
            }
            buf.push_str("\n")
        }
        write!(f,"{}", buf)
    }
}

impl fmt::Debug for Game<Cell> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for row in self.grid.iter() {
            for cell in row.iter() {
                buf.push_str( format!(" {:?}", cell).as_str() );
            }
            buf.push_str("\n")
        }
        write!(f,"{}", buf)
    }
}
