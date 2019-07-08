use std::fmt::{Debug,Display,Result,Formatter};

// default characters used for printing a cell "state" to an output stream (like stdio)
const MINE: char        = '\u{25CF}';   // black circle \u{25CF}
const REVEALED: char    = '\u{2610}';   // ballot box
const HIDDEN: char      = '\u{25A1}';   // white square
const QUESTION: char    = '\u{003F}';   // question mark
const FLAG: char        = '⚑';         // black flag

#[derive(PartialEq)]
pub enum CellState {
    Revealed,
    Marked(CellMarker),
    None
}

#[derive(PartialEq)]
pub enum CellMarker {
    Flagged,
    Questioned,
}

#[derive(PartialEq)]
pub enum CellKind {
    Mine,
    Empty
}

#[derive(PartialEq)]
pub struct GridCell {
    state: CellState,
    kind: CellKind,
    adj_mine_count: u8
}

pub trait MineSweeperCell {

    fn marker(&self) -> Option<CellMarker>;
    fn set_marker(&mut self, marker: CellMarker);
    fn set_kind(&mut self, state: CellKind);
    fn kind(&self) -> &CellKind;
    fn set_state(&mut self, state: CellState);
    fn state(&self) -> &CellState;
    fn adj_mine_count(&self) -> u8;
    fn set_adj_mine_count(&mut self, count: u8);

    /// a cell that is empty and has an adjacent mine count = 0
    fn is_lone_cell(&self) -> bool;
    /// is the cell currently flagged
    fn is_flagged(&self) -> bool;
}

impl GridCell {
    // create an empty cell
    pub fn new(kind :CellKind) -> GridCell {
        GridCell {
            state: CellState::None,
            kind: kind,
            adj_mine_count: 0,
        }
    }

}

impl MineSweeperCell for GridCell {

    fn marker(&self) -> Option<CellMarker> {
        match self.state {
            CellState::Marked(CellMarker::Flagged) => Some(CellMarker::Flagged),
            CellState::Marked(CellMarker::Questioned) => Some(CellMarker::Questioned),
            _ => None
        }
    }

    fn set_marker(&mut self, marker: CellMarker) {
        self.state = CellState::Marked(marker);
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

    fn is_flagged(&self) -> bool {
        self.state == CellState::Marked(CellMarker::Flagged)
    }
}

/// Prints the cell and takes into account whether or not the cell has been revealed or marked.
/// This is the default method to use for displaying a cell
impl Display for GridCell {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let cell_char = match self.state {
            CellState::Revealed => {
                match self.kind {
                    CellKind::Mine => MINE,
                    CellKind::Empty if self.adj_mine_count > 0 => (self.adj_mine_count + 48) as char,
                    _ => REVEALED,
                }
            },
            CellState::Marked(CellMarker::Flagged) => FLAG,
            CellState::Marked(CellMarker::Questioned) => QUESTION,
            CellState::None => HIDDEN
        };
        write!(f,"{}", cell_char)
    }
}

/// Debug will print the cells "kind". It essentially reveals the cell so that you can
/// see the mined cells and also check if the mine counts are correct
impl Debug for GridCell {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let cell_char = match self.kind {
            CellKind::Mine => MINE,
            CellKind::Empty => (self.adj_mine_count + 48) as char,  // convert to ascii digit + 48
        };
        write!(f,"{}", cell_char)
    }
}

