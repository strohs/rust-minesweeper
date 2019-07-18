//! this mod contains the main game logic and game state for Rust Minesweeper.
//!
//! The game "grid" is a wrapper around a 2-dimensional Vec of `GridCell`s
//!

use crate::game_cell::{CellKind, CellMarker, CellState, GameCell, MineSweeperCell};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::fmt;

/// holds the minesweeper grid in a two-dimensional vector
pub struct GameGrid<T: MineSweeperCell> {
    grid: Vec<Vec<T>>,
    elapsed_time: u64,
}

pub trait MineSweeperGame {
    /// initialize a new game of mine sweeper with r rows and c columns
    fn init(r: usize, c: usize) -> Self;

    /// returns the dimensions of the minesweeper grid
    fn dimensions(&self) -> (usize, usize);

    /// returns the row,col indices of GridCells that are mined
    fn mine_indices(&self) -> Vec<(usize, usize)>;

    /// returns a count of the total number of mines in the grid
    fn total_mines(&self) -> usize;

    /// reveals the cell at index r,c
    fn reveal_cell(&mut self, r: usize, c: usize);

    /// reveals all "lone" cells that are connected to the cell at index: r,c.
    /// A lone cell is an `CellKind::Empty` cell with an `adjacent mine count = 0`.
    /// This method essentially does a flood fill, that reveals connected "lone" cells
    fn reveal_all_lone_cells(&mut self, r: usize, c: usize);

    /// place a flag marker at the cell given by the index: r,c
    fn flag_cell(&mut self, r: usize, c: usize);

    /// places a question marker at the cell given by the index: r,c
    fn question_cell(&mut self, r: usize, c: usize);

    /// removes a marker from the cell (if present) given by the index: r,c
    fn unmark_cell(&mut self, r: usize, c: usize);

    /// toggles a cell marker. If a marker is already present at index: r,c then this method
    /// will remove it. If the cell is unmarked, then the CellMarker given by `mark` is placed
    fn toggle_mark(&mut self, r: usize, c: usize, mark: CellMarker);

    /// returns true if the current game is *won. A minesweeper game is won when all mined cells
    /// have been correctly flagged
    fn is_game_won(&self) -> bool;

    /// returns true if a minesweeper game is over (or lost). A game is lost if a user reveals
    /// a mined cell
    fn is_game_lost(&self) -> bool;
}

impl GameGrid<GameCell> {

    /// builds a 2d Vector of empty `GridCell`s
    fn empty_grid(rows: usize, cols: usize) -> Vec<Vec<GameCell>> {
        let mut grid = Vec::with_capacity(rows);
        for _r in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for _c in 0..cols {
                row.push(GameCell::new(CellKind::Empty))
            }
            grid.push(row)
        }
        grid
    }

    /// Generates `count` amount of random (row,col) indices
    /// returns a Vec of tuples containing random (row,col) indices
    fn gen_rand_grid_indices(row_len: usize, col_len: usize, count: usize) -> Vec<(usize, usize)> {
        // build a vec of all grid indices in row major form and shuffle them
        let mut rng = thread_rng();
        let mut grid_indices: Vec<usize> = (0..(row_len * col_len)).map(|i| i).collect();
        grid_indices.shuffle(&mut rng);

        grid_indices
            .into_iter()
            .take(count)
            .map(|i| (i / col_len, i - ((i / col_len) * col_len)))
            .collect::<Vec<(usize, usize)>>()
    }

    /// returns the indices of all grid cells located "around" the cell located at r,c, but
    /// does not include the cell at r,c
    fn adjacent_indices(grid: &Vec<Vec<GameCell>>, r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut adj_ndxs = vec![];
        let max_rows = grid.len();
        let max_cols = grid[0].len();
        let rstart = if r <= 1 { 0 } else { r - 1 };
        let cstart = if c <= 1 { 0 } else { c - 1 };
        let rend = if (r + 1) >= max_rows {
            max_rows - 1
        } else {
            r + 1
        };
        let cend = if (c + 1) >= max_cols {
            max_cols - 1
        } else {
            c + 1
        };

        for nr in rstart..=rend {
            for nc in cstart..=cend {
                // push all the cells located around index: r,c  into the return vector
                if !(nr == r && nc == c) {
                    adj_ndxs.push((nr, nc));
                }
            }
        }
        adj_ndxs
    }

    /// returns a tuple of grid indices that are connected to the cell at r,c AND that
    /// are "lone cells". Lone cells are cells that are not adjacent to any mines
    /// This is an implementation of the flood fill algorithm using depth first search
    fn connected_lone_cell_indices(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut visited = vec![]; // cells already visited
        let mut to_visit = vec![(r, c)]; // cells left to visit
        let mut connected_ndxs = vec![]; // holds the connected cell indices

        while !to_visit.is_empty() {
            let (cr, cc) = to_visit.pop().unwrap();

            if visited.contains(&(cr, cc)) {
                continue;
            } else {
                // add lone cell indices to the list of connected cell indices
                if self.grid[cr][cc].is_lone_cell() {
                    connected_ndxs.push((cr, cc));
                }

                // mark the current cell as visited
                visited.push((cr, cc));

                // build a list of "lone" cells adjacent to the current cell
                let mut adj_ndxs = GameGrid::adjacent_indices(&self.grid, cr, cc)
                    .into_iter()
                    .filter(|(r, c)| self.grid[*r][*c].is_lone_cell())
                    .collect::<Vec<(usize, usize)>>();
                to_visit.append(&mut adj_ndxs);
            }
        }
        connected_ndxs
    }
}

impl MineSweeperGame for GameGrid<GameCell> {

    fn init(num_rows: usize, num_cols: usize) -> Self {
        let mut grid = GameGrid::empty_grid(num_rows, num_cols);

        // generate random mine locations
        let total_mines = ((num_rows * num_cols) as f32 * 0.15f32).round() as usize;
        let mine_ndxs = GameGrid::gen_rand_grid_indices(num_rows, num_cols, total_mines);
        for (r, c) in mine_ndxs.iter() {
            grid[*r][*c] = GameCell::new(CellKind::Mine);
        }

        // compute the adjacent mine counts for every cell that contains a mine
        for (r, c) in mine_ndxs.iter() {
            for (ar, ac) in GameGrid::adjacent_indices(&grid, *r, *c) {
                let cur_count = grid[ar][ac].adj_mine_count() + 1;
                grid[ar][ac].set_adj_mine_count(cur_count);
            }
        }

        GameGrid {
            grid,
            elapsed_time: 0,
        }
    }

    fn dimensions(&self) -> (usize, usize) {
        (self.grid.len(), self.grid[0].len())
    }

    fn mine_indices(&self) -> Vec<(usize, usize)> {
        let mut locations = vec![];
        for (r, row) in self.grid.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if *cell.kind() == CellKind::Mine {
                    locations.push((r, c))
                }
            }
        }
        locations
    }

    /// TOTAL_MINES = grid.rows * grid.columns * 0.15
    fn total_mines(&self) -> usize {
        let row_len = self.grid.len();
        let col_len = self.grid[0].len();
        ((row_len * col_len) as f32 * 0.15f32).round() as usize
    }

    fn reveal_cell(&mut self, r: usize, c: usize) {
        if *self.grid[r][c].state() != CellState::Revealed {
            self.grid[r][c].set_state(CellState::Revealed);
            self.reveal_all_lone_cells(r, c);
        }
    }

    fn reveal_all_lone_cells(&mut self, r: usize, c: usize) {
        let connected_ndxs = self.connected_lone_cell_indices(r, c);

        // also reveal all the cells that are adjacent to the lone cells
        let adj_perimeter_cells: HashSet<(usize, usize)> = connected_ndxs
            .iter()
            .flat_map(|(cr, cc)| GameGrid::adjacent_indices(&self.grid, *cr, *cc))
            .collect();

        for (ri, ci) in connected_ndxs {
            self.reveal_cell(ri, ci);
        }
        for (ri, ci) in adj_perimeter_cells {
            self.reveal_cell(ri, ci);
        }
    }

    fn flag_cell(&mut self, r: usize, c: usize) {
        if *self.grid[r][c].state() != CellState::Revealed {
            self.grid[r][c].set_state(CellState::Marked(CellMarker::Flagged));
        }
    }

    fn question_cell(&mut self, r: usize, c: usize) {
        if *self.grid[r][c].state() != CellState::Revealed {
            self.grid[r][c].set_state(CellState::Marked(CellMarker::Questioned));
        }
    }

    fn unmark_cell(&mut self, r: usize, c: usize) {
        if *self.grid[r][c].state() != CellState::Revealed {
            self.grid[r][c].set_state(CellState::Unknown);
        }
    }

    fn toggle_mark(&mut self, r: usize, c: usize, mark: CellMarker) {
        if let CellState::Marked(_) = self.grid[r][c].state() {
            self.unmark_cell(r, c);
        } else {
            self.grid[r][c].set_state(CellState::Marked(mark));
        }
    }

    fn is_game_won(&self) -> bool {
        self.mine_indices()
            .iter()
            .all(|(r, c)| self.grid[*r][*c].is_flagged())
    }

    fn is_game_lost(&self) -> bool {
        self.mine_indices()
            .iter()
            .any(|(r, c)| *self.grid[*r][*c].state() == CellState::Revealed)
    }
}

impl fmt::Display for GameGrid<GameCell> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for row in self.grid.iter() {
            for cell in row.iter() {
                buf.push_str(format!(" {}", cell).as_str());
            }
            buf.push_str("\n")
        }
        write!(f, "{}", buf)
    }
}

impl fmt::Debug for GameGrid<GameCell> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for row in self.grid.iter() {
            for cell in row.iter() {
                buf.push_str(format!(" {:?}", cell).as_str());
            }
            buf.push_str("\n")
        }
        write!(f, "{}", buf)
    }
}
