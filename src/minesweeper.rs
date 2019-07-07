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
    fn init(&mut self);
    fn mine_locations(&self) -> Vec<(usize,usize)>;
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

    /// returns the indices of all cells located "around" the cell located at r,c
    pub fn adjacent_indices(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut ndxs = vec![];
        let max_rows = self.grid.len();
        let max_cols = self.grid[0].len();
        let rstart = if r <= 1 { 0 } else { r - 1 };
        let cstart = if c <= 1 { 0 } else { c - 1 };
        let rend = if (r + 1) >= max_rows { max_rows-1 } else { r + 1 };
        let cend = if (c + 1) >= max_cols { max_cols-1 } else { c + 1 };
        println!("[{}:{}] [{}:{}]", rstart,rend, cstart, cend );

        for nr in rstart..=rend {
            for nc in cstart..=cend {
                // push all the cells located around index: r,c  into the return vector
                if nr != r || nc != c {
                    ndxs.push((nr, nc));
                }
            }
        }
        ndxs
    }

    pub fn connected_cell_indices(&self, r: usize, c: usize) -> Vec<(usize,usize)> {
        let mut visited = vec![];
        let mut to_visit = vec![(r,c)];
        let mut connected_ndxs = vec![];

        while !to_visit.is_empty() {
            let (cr,cc) = to_visit.pop().unwrap();

            if visited.contains(&(cr,cc) ) {
                continue;    
            } else {
                // if the current cell is a "lone" cell add it to the list of connected cells
                if self.grid[cr][cc].is_lone_cell() { connected_ndxs.push( (cr,cc) ); }

                // add the current cell to the already visited list
                visited.push((cr,cc) );

                // get a list of "lone" cells adjacent to the current cell
                let mut adj_ndxs = self.adjacent_indices(cr,cc)
                    .into_iter()
                    .filter(|(r, c)| {
                        self.grid[*r][*c].is_lone_cell()
                    } )
                    .collect::<Vec<(usize,usize)>>();
                to_visit.append( &mut adj_ndxs);
            }
        }
        connected_ndxs
    }
}

impl MineSweeperGame for Game<Cell> {

    fn init(&mut self) {
        self.randomize_mine_locations();

        // set the adjacent mine counts for each cell
        for (r,c) in self.mine_locations() {
            self.adjacent_cells(r,c)
                .iter_mut()
                .filter(|cell| *cell.kind() != CellKind::Mine )
                .for_each(|cell| {
                    let mine_count = cell.adj_mine_count() + 1;
                    cell.set_adj_mine_count( mine_count );
                })
        }
    }

    fn mine_locations(&self) -> Vec<(usize, usize)> {
        let mut locations = vec![];
        for (r, row) in self.grid.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if *cell.kind() == CellKind::Mine { locations.push( (r,c) ) }
            }
        }
        locations
    }

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
        let neighbor_ndxs = self.adjacent_indices(r, c);
        let mut neighbors = vec![];
        for (ri, row) in self.grid.iter_mut().enumerate() {
            for (ci,cell) in row.iter_mut().enumerate() {
                if neighbor_ndxs.contains(&(ri,ci) ) {
                    neighbors.push(cell);
                }
            }
        }
        neighbors
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
