Rust Minesweeper
=======================================================
This is a command line driven minesweeper clone. One of the first Rust programs I wrote.

## Running
> cargo run --bin minesweeper


The board will be drawn to the terminal using ASCII graphics. You will then need to make a move using one
of the following commands:

* to create a new game with 5 rows and 5 columns: `n 5 5`
* to reveal a square at row 0 column 1: `r 0 1`
* to flag a square at row 2 column 4: `f 2 4`
* to place a question mark on a square at row 1 column 3: `q 1 3`


The game will end if you reveal a square with a mine in it, or if you successfully flag all squares containing
a mine.