use std::fmt;


enum CellMarker {
    Flag,
    Question
}

enum CellState {
    Revealed,
    Marked(CellMarker),
    Clear
}

enum CellType {
    Mine,
    Empty
}

struct Cell {
    icon: char,
    state: CellState,
    typ: CellType
}

impl Cell {
    fn empty() -> Cell {
        Cell {
            icon: '*',
            state: CellState::Clear,
            typ: CellType::Empty
        }
    }
}

struct Minefield {
     board: Vec<Vec<Cell>>
}

impl Minefield {
    fn new(rows: usize, cols:usize) -> Minefield {
        let mut board: Vec<Vec<Cell>> = vec![vec![]];
        for r in 0..rows {
            let mut row = vec![];
            for c in 0..cols {
                row.push(Cell::empty());
            }
            board.push(row)
        }
        Minefield {
            board
        }
    }
}

impl fmt::Display for Minefield {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for row in self.board.iter() {
            for cell in row.iter() {
                buf.push_str( format!(" {}", cell.icon).as_str() );
            }
            buf.push_str("\n")
        }
        write!(f,"{}", buf)
    }
}

fn main() {
    let mut mf = Minefield::new(5,3);
    println!("{}", mf);
    mf.board[1][1].icon = '!';
}
