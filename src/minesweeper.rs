//! this is the main game logic and state for Rust Minesweeper.
//!
//! The game "grid" is maintained in a 2-dimensional Vec of `GridCell`s which is then
//! contained within the `Game` struct

use std::fmt;
use rand::{thread_rng};
use rand::seq::{SliceRandom};
use crate::grid_cell::{GridCell, MineSweeperCell, CellKind, CellState, CellMarker};
use std::collections::HashSet;

pub struct Game<T: MineSweeperCell> {
    grid: Vec<Vec<T>>,
    elapsed_time: u64,
}

pub trait MineSweeperGame {
    fn init(r: usize, c: usize) -> Self;
    fn dimensions(&self) -> (usize, usize);
    fn mine_indices(&self) -> Vec<(usize, usize)>;
    fn total_mines(&self) -> usize;
    fn reveal_cell(&mut self, r: usize, c:usize);
    fn reveal_connected_cells(&mut self, r: usize, c: usize);
    fn flag_cell(&mut self, r:usize, c:usize);
    fn question_cell(&mut self, r: usize, c:usize);
    fn unmark_cell(&mut self, r: usize, c:usize);
    fn toggle_mark(&mut self, r: usize, c: usize, mark: CellMarker);
    fn is_game_won(&self) -> bool;
    fn is_game_over(&self) -> bool;
}

impl Game<GridCell> {

    /// builds a 2d Vector of empty `GridCell`
    fn empty_grid(rows: usize, cols:usize) -> Vec<Vec<GridCell>> {
        let mut grid = Vec::with_capacity(rows);
        for _r in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for _c in 0..cols {
                row.push(GridCell::new(CellKind::Empty))
            }
            grid.push(row)
        }
        grid
    }

    /// returns a Vec of tuples containing random (row,col) indices
    ///
    fn random_grid_indices(row_len: usize, col_len: usize, count: usize) -> Vec<(usize,usize)> {
        // build a vec of all grid indices in row major form and shuffle them
        let mut rng = thread_rng();
        let mut grid_indices: Vec<usize> = (0..(row_len * col_len)).map(|i| i).collect();
        grid_indices.shuffle(&mut rng);

        grid_indices.into_iter()
            .take(count)
            .map(|i| (i / col_len, i - ((i / col_len) * col_len) ))
            .collect::<Vec<(usize, usize)>>()
    }

    /// returns the indices of all grid cells located "around" the cell located at r,c, but
    /// does not include the cell at r,c
    fn adjacent_indices(grid: &Vec<Vec<GridCell>>, r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut ndxs = vec![];
        let max_rows = grid.len();
        let max_cols = grid[0].len();
        let rstart = if r <= 1 { 0 } else { r - 1 };
        let cstart = if c <= 1 { 0 } else { c - 1 };
        let rend = if (r + 1) >= max_rows { max_rows-1 } else { r + 1 };
        let cend = if (c + 1) >= max_cols { max_cols-1 } else { c + 1 };

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

    /// returns a tuple of grid indices that are connected to the cell at r,c AND that
    /// are "lone cells". Lone cells are cells that are not adjacent to any mines
    /// This is an implementation of the flood fill algorithm using depth first search
    fn connected_lone_cell_indices(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut visited = vec![];        // cells already visited
        let mut to_visit = vec![(r,c)];  // cells left to visit
        let mut connected_ndxs = vec![]; // holds the connected cell indices

        while !to_visit.is_empty() {
            let (cr,cc) = to_visit.pop().unwrap();

            if visited.contains(&(cr,cc) ) {
                continue;    
            } else {
                // if the current cell is a "lone" cell add it to the list of connected cells
                if self.grid[cr][cc].is_lone_cell() { connected_ndxs.push( (cr,cc) ); }

                // add the current cell to the already visited list
                visited.push((cr,cc) );

                // build a list of "lone" cells adjacent to the current cell
                let mut adj_ndxs = Game::adjacent_indices(&self.grid, cr,cc)
                    .into_iter()
                    .filter(|(r, c)| {
                        self.grid[*r][*c].is_lone_cell()
                    })
                    .collect::<Vec<(usize,usize)>>();
                to_visit.append( &mut adj_ndxs);
            }
        }

        connected_ndxs
    }
}

impl MineSweeperGame for Game<GridCell> {

    fn init(r: usize, c: usize) -> Self {
        let mut grid = Game::empty_grid(r, c);

        // generate random mine locations
        let total_mines = ((r * c) as f32 * 0.15f32).round() as usize;
        let mine_ndxs = Game::random_grid_indices(r,c, total_mines);
        for (ri, ci) in mine_ndxs.iter() {
            grid[*ri][*ci] = GridCell::new(CellKind::Mine );
        }

        // set the adjacent mine counts for every cell that contains a mine
        for (ri,ci) in mine_ndxs.iter() {
            for (ari, aci) in Game::adjacent_indices( &grid, *ri, *ci ) {
                let cur_count = grid[ari][aci].adj_mine_count() + 1;
                grid[ari][aci].set_adj_mine_count( cur_count );
            }
        }

        Game {
            grid,
            elapsed_time: 0
        }
    }

    /// returns the dimensions of the game grid as (row, col)
    fn dimensions(&self) -> (usize, usize) {
        (self.grid.len(), self.grid[0].len() )
    }

    /// returns the row,col indices of mined cells
    fn mine_indices(&self) -> Vec<(usize, usize)> {
        let mut locations = vec![];
        for (r, row) in self.grid.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if *cell.kind() == CellKind::Mine { locations.push( (r,c) ) }
            }
        }
        locations
    }

    /// computes the total mines that should be on a grid, using the grid's dimensions
    /// TOTAL_MINES = grid.rows * grid.columns * 0.15
    fn total_mines(&self) -> usize {
        let row_len = self.grid.len();
        let col_len = self.grid[0].len();
        ((row_len * col_len) as f32 * 0.15f32).round() as usize
    }

    /// reveals the grid cell located at index r,c. This method will also reveal any "lone"
    /// cells connected to this cell.
    fn reveal_cell(&mut self, r: usize, c: usize) {
        if *self.grid[r][c].state() != CellState::Revealed {
            self.grid[r][c].set_state( CellState::Revealed );
            self.reveal_connected_cells(r,c);
        }
    }

    /// reveal all empty grid cells that are connected to this cell BUT aren't adjacent
    /// to any mines
    fn reveal_connected_cells(&mut self, r: usize, c: usize) {
        let connected_ndxs = self.connected_lone_cell_indices(r, c);

        // also reveal all the cells that are adjacent to the lone cells
        let adj_perimeter_cells: HashSet<(usize, usize)> = connected_ndxs.iter()
            .flat_map(|(cr,cc)| Game::adjacent_indices(&self.grid, *cr, *cc) )
            .collect();

        for (ri,ci) in connected_ndxs {
            self.reveal_cell(ri,ci);
        }
        for (ri,ci) in adj_perimeter_cells {
            self.reveal_cell(ri,ci);
        }
    }

    /// marks the cell at index r,c as flagged
    fn flag_cell(&mut self, r: usize, c: usize) {
        if *self.grid[r][c].state() != CellState::Revealed {
            self.grid[r][c].set_state( CellState::Marked(CellMarker::Flagged) );
        }
    }

    /// marks the cell at index r,c as questioned
    fn question_cell(&mut self, r: usize, c: usize) {
        if *self.grid[r][c].state() != CellState::Revealed {
            self.grid[r][c].set_state( CellState::Marked(CellMarker::Questioned) );
        }
    }

    /// clears any existing marks on a cell and reverts the cell's state to None (aka unknown)
    fn unmark_cell(&mut self, r: usize, c: usize) {
        if *self.grid[r][c].state() != CellState::Revealed {
            self.grid[r][c].set_state( CellState::Unknown);
        }
    }

    fn toggle_mark(&mut self, r: usize, c: usize, mark: CellMarker) {
        if let CellState::Marked(_) = self.grid[r][c].state() {
            self.unmark_cell(r, c);
        } else {
            self.grid[r][c].set_state( CellState::Marked(mark));
        }
    }

    /// checks if the current game is won and returns true else false
    /// A game is won if all cell containing a mine have been flagged
    fn is_game_won(&self) -> bool {
        self.mine_indices()
            .iter()
            .all(|(r,c)| self.grid[*r][*c].is_flagged() )
    }

    /// checks if a game is over and returns true if it is, otherwise returns false
    /// A game is over once a mined cell has been revealed
    fn is_game_over(&self) -> bool {
        self.mine_indices()
            .iter()
            .any(|(r,c)| *self.grid[*r][*c].state() == CellState::Revealed)
    }
}

impl fmt::Display for Game<GridCell> {
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

impl fmt::Debug for Game<GridCell> {
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
