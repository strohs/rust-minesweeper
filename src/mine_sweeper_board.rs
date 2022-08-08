//! This module contains the components that make up a game of minesweeper played on a 2D grid
//!

// default characters printing game cells to standard output
pub const MINE: char = '\u{25CF}'; // UTF-8 black circle \u{25CF}
pub const REVEALED: char = '0'; // UTF-8 ballot box \u{2610}
pub const HIDDEN: char = '\u{25A1}'; // UTF-8 white square
pub const QUESTION: char = '\u{003F}'; // question mark
pub const FLAG: char = '⚑'; // UTF-8 black flag \u{2691}

#[derive(PartialEq)]
/// holds information on the current state of a MineSweeper cell
/// `Revealed` - a user has revealed the cell
/// `Marked` - a user has "marked" a cell with either a Flag or Question Mark
/// `Hidden` - the cell has not yet been revealed by the user
pub enum CellState {
    Revealed,
    Marked(CellMarker),
    Hidden,
}

#[derive(PartialEq)]
/// holds information about whether or not a Cell is currently "marked" with a Flag, or  Question mark
pub enum CellMarker {
    Flagged,
    Questioned,
}

#[derive(PartialEq)]
/// the "kind" of cell, either the Cell is mined, or it is empty
pub enum CellKind {
    Mine,
    Empty,
}

/// The Basic "building-block" of a game of MineSweeper is a cell. Cells can have a mine in them, or
/// be empty. Additionally, they can also be "marked" with a flag or a question mark. If a cell is
/// next to one or more mines than the cell's adj_mine_count field will contain a count of the
/// number of mines adjacent to the cell.
pub trait MineSweeperCell {
    /// return the cells current marker if any, else returns None if the cells is not marked
    fn marker(&self) -> Option<CellMarker>;

    /// set the cell's Marker (either flagged or questioned)
    fn set_marker(&mut self, marker: CellMarker);

    /// is the cell currently flagged
    fn is_flagged(&self) -> bool;

    /// set the Cell Kind
    fn set_kind(&mut self, state: CellKind);

    /// returns the cells kind,, either Mine or Empty
    fn kind(&self) -> &CellKind;

    /// sets the cells state ( revealed, flagged, unknown...)
    fn set_state(&mut self, state: CellState);

    /// returns the cells current state
    fn state(&self) -> &CellState;

    /// returns the count of the number of mines adjacent to this cell
    fn adj_mine_count(&self) -> u8;

    /// sets the adjacent mine count of this cell
    fn set_adj_mine_count(&mut self, count: u8);

    /// a cell that is empty and has an adjacent mine count = 0
    fn is_lone_cell(&self) -> bool;
}

/// MineSweeperGame
/// this is the main trait listing the functions required for playing a game of MineSweeper on
/// a 2D Grid of `MineSweeperCell`s
pub trait MineSweeperGame {
    /// initialize a new mine sweeper grid with r rows and c columns
    fn init(r: usize, c: usize) -> Self;

    /// returns the dimensions of the minesweeper grid
    fn dimensions(&self) -> (usize, usize);

    /// returns the row,col indices of GridCells that are mined
    fn mine_indices(&self) -> Vec<(usize, usize)>;

    /// returns a count of the total number of mines in the grid
    fn total_mines(&self) -> usize;

    /// reveals the cell at row index `r` and column index `c`
    fn reveal_cell(&mut self, r: usize, c: usize);

    /// reveals all "lone" cells that are connected to the cell at index `r`,`c`
    /// A lone cell is a `CellKind::Empty` cell with an `adjacent mine count = 0`.
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

    /// returns `true` if the current game is won. A minesweeper game is won when all mined cells
    /// have been correctly flagged
    fn is_game_won(&self) -> bool;

    /// returns `true` if a minesweeper game is lost. A game is lost if a user reveals
    /// a mined cell
    fn is_game_lost(&self) -> bool;
}
