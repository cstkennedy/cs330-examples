use itertools::Itertools;
use std::fmt;

pub const VALID_SYMBOLS: [char; 2] = ['X', 'O'];

/// This ADT represents the gameboard used in a round
/// of standard tic-tac-toe (i.e., a 3 x 3 grid)
/// <p>
/// Each entry in the Board is referred to as a _Cell_.
/// A Cell can be empty, where it stores a value in the range 1-9
/// The digit represents the cell id and is used to update a Cell
#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    the_board: [char; 9],
}

impl Default for Board {
    fn default() -> Self {
        Board {
            the_board: ['1', '2', '3', '4', '5', '6', '7', '8', '9'],
        }
    }
}

impl Board {
    pub fn new() -> Self {
        Board::default()
    }

    /// Retrieve the value stored in a selected Cell.
    ///
    /// Args:
    ///     cell_id: numeric id representing the desired cell
    ///
    /// Returns:
    ///     value stored in the Cell
    ///
    pub fn get_cell(&self, cell_id: usize) -> Result<char, &'static str> {
        if cell_id <= 0 || cell_id >= 10 {
            return Err("Cell Index is not between 0 and 10, exclusive");
        }

        Ok(self.the_board[cell_id - 1])
    }

    /// Set the value stored in a selected Cell.
    ///
    /// Args:
    ///     cell_id: numeric id representing the desired cell
    ///     new_value: replacement `CellValue`
    ///
    pub fn set_cell(&mut self, cell_id: usize, new_value: char) -> Result<(), &'static str> {
        if cell_id <= 0 || cell_id >= 10 {
            return Err("Cell Index is not between 0 and 10, exclusive");
        }

        if VALID_SYMBOLS.iter().find(|symbol| *symbol == &new_value) == None {
            return Err("'new_value' is not 'X' or 'O'");
        }

        self.the_board[cell_id - 1] = new_value;
        Ok(())
    }

    /// Get the contents of each row... in order.
    pub fn rows(&self) -> [[char; 3]; 3] {
        [
            [self.the_board[0], self.the_board[1], self.the_board[2]],
            [self.the_board[3], self.the_board[4], self.the_board[5]],
            [self.the_board[6], self.the_board[7], self.the_board[8]],
        ]
    }

    /// Get the contents of each column... in order.
    pub fn columns(&self) -> [[char; 3]; 3] {
        [
            [self.the_board[0], self.the_board[3], self.the_board[6]],
            [self.the_board[1], self.the_board[4], self.the_board[7]],
            [self.the_board[2], self.the_board[5], self.the_board[8]],
        ]
    }

    /// Get the contents of each diagonal... in order.
    pub fn diagonals(&self) -> [[char; 3]; 2] {
        [
            [self.the_board[0], self.the_board[4], self.the_board[8]],
            [self.the_board[2], self.the_board[4], self.the_board[6]],
        ]
    }

    /// Return true if all 9 cells hold player symbols.
    ///
    /// Returns:
    ///     True if every cell in the board has either an 'X' or an 'O'
    pub fn is_full(&self) -> bool {
        self.the_board
            .iter()
            .map(|cell: &char| cell.is_numeric())
            .count()
            == 0
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let board_str = self
            .rows()
            .iter()
            .map(|&row| row.iter().join("|"))
            .join("\n");

        write!(f, "{}", board_str)
    }
}
