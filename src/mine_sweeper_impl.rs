use crate::mine_sweeper_board::{
    CellKind, CellMarker, CellState, MineSweeperCell, MineSweeperGame, FLAG, HIDDEN, MINE,
    QUESTION, REVEALED,
};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;

/// MineSweeper cell
/// holds the state of a cell in a minesweeper grid
#[derive(PartialEq)]
pub struct Cell {
    state: CellState,
    kind: CellKind,
    adj_mine_count: u8,
}

/// MineSweeper Grid.
/// This struct contains a 2D grid of minesweeper cells stored in a 1D vector
pub struct Grid<T: MineSweeperCell> {
    grid: Vec<T>,
    num_rows: usize,
    num_cols: usize,
}

impl Cell {
    /// create a new empty cell, with CellState::Hidden and adjacent mine count of 0
    pub fn new(kind: CellKind) -> Cell {
        Cell {
            state: CellState::Hidden,
            kind,
            adj_mine_count: 0,
        }
    }
}

impl MineSweeperCell for Cell {
    fn marker(&self) -> Option<CellMarker> {
        match self.state {
            CellState::Marked(CellMarker::Flagged) => Some(CellMarker::Flagged),
            CellState::Marked(CellMarker::Questioned) => Some(CellMarker::Questioned),
            _ => None,
        }
    }

    fn set_marker(&mut self, marker: CellMarker) {
        self.state = CellState::Marked(marker);
    }

    fn is_flagged(&self) -> bool {
        self.state == CellState::Marked(CellMarker::Flagged)
    }

    fn set_kind(&mut self, kind: CellKind) {
        self.kind = kind;
    }

    fn kind(&self) -> &CellKind {
        &self.kind
    }

    fn set_state(&mut self, state: CellState) {
        self.state = state;
    }

    fn state(&self) -> &CellState {
        &self.state
    }

    fn adj_mine_count(&self) -> u8 {
        self.adj_mine_count
    }

    fn set_adj_mine_count(&mut self, count: u8) {
        self.adj_mine_count = count
    }

    fn is_lone_cell(&self) -> bool {
        self.kind == CellKind::Empty && self.adj_mine_count == 0
    }
}

/// Prints the cell and takes into account whether or not the cell has been revealed or marked.
/// This method is used to display the gridCell during a game of MineSweeper
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let cell_char = match self.state {
            CellState::Revealed => match self.kind {
                CellKind::Mine => MINE,
                CellKind::Empty if self.adj_mine_count > 0 => (self.adj_mine_count + 48) as char,
                _ => REVEALED,
            },
            CellState::Marked(CellMarker::Flagged) => FLAG,
            CellState::Marked(CellMarker::Questioned) => QUESTION,
            CellState::Hidden => HIDDEN,
        };
        write!(f, "{}", cell_char)
    }
}

/// Debug will print the cell's `CellKind` value. It temporarily "reveals" the cell so that you can
/// see the mined cells and also check if the mine counts are correct
impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let cell_char = match self.kind {
            CellKind::Mine => MINE,
            CellKind::Empty => (self.adj_mine_count + 48) as char, // convert to ASCII digit by adding + 48
        };
        write!(f, "{}", cell_char)
    }
}

impl Grid<Cell> {
    /// builds a Vector of empty `GridCell`s
    fn empty_grid(rows: usize, cols: usize) -> Vec<Cell> {
        let mut grid = Vec::with_capacity(rows * cols);
        for _i in 0..(rows * cols) {
            grid.push(Cell::new(CellKind::Empty));
        }
        grid
    }

    /// Generates `count` amount of random grid indices and returns them in a Vector<usize>
    fn gen_rand_grid_indices(row_len: usize, col_len: usize, count: usize) -> Vec<usize> {
        // build a vec of all grid indices in row major form and shuffle them
        let mut rng = thread_rng();
        let mut grid_indices: Vec<usize> = (0..(row_len * col_len)).collect();
        grid_indices.shuffle(&mut rng);
        grid_indices.into_iter().take(count).collect()
    }

    /// returns the **indices** of all grid cells "adjacent" to the cell located at `index`, but
    /// does not include the cell at `index`
    fn adjacent_indices(num_rows: usize, num_cols: usize, index: usize) -> Vec<usize> {
        let mut adj_ndxs = vec![];
        let r = index / num_cols;
        let c = index % num_cols;
        let rstart = if r <= 1 { 0 } else { r - 1 };
        let cstart = if c <= 1 { 0 } else { c - 1 };
        let rend = if (r + 1) >= num_rows {
            num_rows - 1
        } else {
            r + 1
        };
        let cend = if (c + 1) >= num_cols {
            num_cols - 1
        } else {
            c + 1
        };

        for nr in rstart..=rend {
            for nc in cstart..=cend {
                // push all the cells located around index: r,c  into the return vector
                if !(nr == r && nc == c) {
                    adj_ndxs.push(nr * num_cols + nc);
                }
            }
        }
        adj_ndxs
    }

    /// returns grid indices that are connected to the cell at `index` AND that
    /// are "lone cells". Lone cells are cells that are not adjacent to any mines
    /// This function is essentially an implementation of flood fill algorithm using depth first search
    fn connected_lone_cell_indices(&self, index: usize) -> Vec<usize> {
        let mut visited = vec![]; // cells already visited
        let mut to_visit = vec![index]; // cells left to visit
        let mut connected_ndxs = vec![]; // holds the connected cell indices

        while !to_visit.is_empty() {
            // current index being visited
            let cur_ndx = to_visit.pop().unwrap();

            if visited.contains(&cur_ndx) {
                continue;
            } else {
                // add lone cell's index to the list of connected cell indices
                if self.grid[cur_ndx].is_lone_cell() {
                    connected_ndxs.push(cur_ndx);
                }

                // mark the current cell as visited
                visited.push(cur_ndx);

                // build a list of "lone" cells adjacent to the current cell
                let mut adj_ndxs = Grid::adjacent_indices(self.num_rows, self.num_cols, cur_ndx)
                    .into_iter()
                    .filter(|ndx| self.grid[*ndx].is_lone_cell())
                    .collect::<Vec<usize>>();
                to_visit.append(&mut adj_ndxs);
            }
        }
        connected_ndxs
    }

    /// translates a two-dimensional row, column index into a one-dimensional index
    fn to_1d(&self, row: usize, column: usize) -> usize {
        row * self.num_cols + column
    }
}

impl MineSweeperGame for Grid<Cell> {
    fn init(num_rows: usize, num_cols: usize) -> Self {
        let mut grid = Grid::empty_grid(num_rows, num_cols);

        // generate random mine locations
        let total_mines = ((num_rows * num_cols) as f32 * 0.15f32).round() as usize;
        let mine_ndxs = Grid::gen_rand_grid_indices(num_rows, num_cols, total_mines);
        for index in mine_ndxs.iter() {
            grid[*index] = Cell::new(CellKind::Mine);
        }

        // compute the adjacent mine counts for every cell that contains a mine
        for index in mine_ndxs.iter() {
            for adj_ndx in Grid::adjacent_indices(num_rows, num_cols, *index) {
                let cur_count = grid[adj_ndx].adj_mine_count() + 1;
                grid[adj_ndx].set_adj_mine_count(cur_count);
            }
        }

        Grid {
            grid,
            num_rows,
            num_cols,
        }
    }

    fn dimensions(&self) -> (usize, usize) {
        (self.num_rows, self.num_cols)
    }

    fn mine_indices(&self) -> Vec<(usize, usize)> {
        self.grid
            .iter()
            .enumerate()
            .filter(|(_ndx, cell)| *cell.kind() == CellKind::Mine)
            .map(|(ndx, _cell)| (ndx / self.num_cols, ndx % self.num_cols))
            .collect::<Vec<(usize, usize)>>()
    }

    /// The number of total mines on a grid is set using the following:
    /// TOTAL_MINES = grid.rows * grid.columns * 0.15
    fn total_mines(&self) -> usize {
        ((self.num_rows * self.num_cols) as f32 * 0.15f32).round() as usize
    }

    fn reveal_cell(&mut self, r: usize, c: usize) {
        let index = self.to_1d(r, c);
        if *self.grid[index].state() != CellState::Revealed {
            self.grid[index].set_state(CellState::Revealed);
            self.reveal_all_lone_cells(r, c);
        }
    }

    fn reveal_all_lone_cells(&mut self, r: usize, c: usize) {
        let connected_ndxs = self.connected_lone_cell_indices(self.to_1d(r, c));

        // also reveal all the cells that are adjacent to the lone cells
        let adj_perimeter_cells: HashSet<usize> = connected_ndxs
            .iter()
            .flat_map(|ndx| Grid::adjacent_indices(self.num_rows, self.num_cols, *ndx))
            .collect();

        for ndx in connected_ndxs {
            self.reveal_cell(ndx / self.num_cols, ndx % self.num_cols);
        }
        for ndx in adj_perimeter_cells {
            self.reveal_cell(ndx / self.num_cols, ndx % self.num_cols);
        }
    }

    fn flag_cell(&mut self, r: usize, c: usize) {
        let index = self.to_1d(r, c);
        if *self.grid[index].state() != CellState::Revealed {
            self.grid[index].set_state(CellState::Marked(CellMarker::Flagged));
        }
    }

    fn question_cell(&mut self, r: usize, c: usize) {
        let index = self.to_1d(r, c);
        if *self.grid[index].state() != CellState::Revealed {
            self.grid[index].set_state(CellState::Marked(CellMarker::Questioned));
        }
    }

    fn unmark_cell(&mut self, r: usize, c: usize) {
        let index = self.to_1d(r, c);
        if *self.grid[index].state() != CellState::Revealed {
            self.grid[index].set_state(CellState::Hidden);
        }
    }

    fn toggle_mark(&mut self, r: usize, c: usize, mark: CellMarker) {
        let index = self.to_1d(r, c);
        if let CellState::Marked(_) = self.grid[index].state() {
            self.unmark_cell(r, c);
        } else {
            self.grid[index].set_state(CellState::Marked(mark));
        }
    }

    fn is_game_won(&self) -> bool {
        self.mine_indices().iter().all(|(r, c)| {
            let index = self.to_1d(*r, *c);
            self.grid[index].is_flagged()
        })
    }

    fn is_game_lost(&self) -> bool {
        self.mine_indices().iter().any(|(r, c)| {
            let index = self.to_1d(*r, *c);
            *self.grid[index].state() == CellState::Revealed
        })
    }
}

impl fmt::Display for Grid<Cell> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for ri in 0..self.num_rows {
            for ci in 0..self.num_cols {
                let index = self.to_1d(ri, ci);
                buf.push_str(format!(" {}", self.grid[index]).as_str());
            }
            buf.push('\n')
        }
        write!(f, "{}", buf)
    }
}

impl fmt::Debug for Grid<Cell> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for ri in 0..self.num_rows {
            for ci in 0..self.num_cols {
                let index = self.to_1d(ri, ci);
                buf.push_str(format!(" {:?}", self.grid[index]).as_str());
            }
            buf.push('\n')
        }
        write!(f, "{}", buf)
    }
}

#[cfg(test)]
mod tests {
    use crate::mine_sweeper_board::{
        CellKind, CellMarker, CellState, MineSweeperCell, FLAG, HIDDEN, MINE, QUESTION, REVEALED,
    };
    use crate::mine_sweeper_impl::Cell;

    #[test]
    fn revealed_mined_cell_should_display_as_mine_char() {
        let mined_cell = Cell {
            state: CellState::Revealed,
            kind: CellKind::Mine,
            adj_mine_count: 0,
        };
        assert_eq!(format!("{}", mined_cell), MINE.to_string());
    }

    #[test]
    fn hidden_cell_should_display_as_hidden_char() {
        let mined_cell = Cell {
            state: CellState::Hidden,
            kind: CellKind::Mine,
            adj_mine_count: 0,
        };
        assert_eq!(format!("{}", mined_cell), HIDDEN.to_string());
    }

    #[test]
    fn revealed_empty_cell_with_no_adjacent_mines_should_display_revealed_char() {
        let cell = Cell {
            state: CellState::Revealed,
            kind: CellKind::Empty,
            adj_mine_count: 0,
        };
        assert_eq!(format!("{}", cell), REVEALED.to_string());
    }

    #[test]
    fn revealed_empty_cell_with_adjacent_mines_should_display_adj_mine_count() {
        let cell = Cell {
            state: CellState::Revealed,
            kind: CellKind::Empty,
            adj_mine_count: 3,
        };
        assert_eq!(format!("{}", cell), "3".to_string());
    }

    #[test]
    fn flagged_cell_displays_the_flag_char() {
        let cell = Cell {
            state: CellState::Marked(CellMarker::Flagged),
            kind: CellKind::Empty,
            adj_mine_count: 1,
        };
        assert_eq!(format!("{}", cell), FLAG.to_string());
    }

    #[test]
    fn questioned_cell_displays_the_question_char() {
        let cell = Cell {
            state: CellState::Marked(CellMarker::Questioned),
            kind: CellKind::Empty,
            adj_mine_count: 1,
        };
        assert_eq!(format!("{}", cell), QUESTION.to_string());
    }

    #[test]
    fn lone_cell_test_should_return_true_for_empty_cell_with_0_adj_mines() {
        let cell = Cell {
            state: CellState::Marked(CellMarker::Flagged),
            kind: CellKind::Empty,
            adj_mine_count: 0,
        };
        assert!(cell.is_lone_cell());
    }

    #[test]
    fn lone_cell_test_should_return_false_for_empty_cell_with_gt_0_adj_mines() {
        let cell = Cell {
            state: CellState::Marked(CellMarker::Flagged),
            kind: CellKind::Empty,
            adj_mine_count: 2,
        };
        assert_eq!(cell.is_lone_cell(), false);
    }

    #[test]
    fn lone_cell_test_should_return_false_for_any_mined_cell() {
        let cell = Cell {
            state: CellState::Marked(CellMarker::Flagged),
            kind: CellKind::Mine,
            adj_mine_count: 2,
        };
        assert_eq!(cell.is_lone_cell(), false);
    }
}
