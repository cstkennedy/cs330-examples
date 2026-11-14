use itertools::Itertools;
use std::fmt;

use crate::error::PositionError;

#[deprecated]
pub const VALID_SYMBOLS: [char; 2] = ['X', 'O'];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Symbol {
    X,
    O,
}

impl Symbol {
    pub fn as_char(&self) -> char {
        match *self {
            Symbol::X => 'X',
            Symbol::O => 'O',
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position(usize);

impl std::ops::Deref for Position {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<usize> for Position {
    type Error = PositionError;

    fn try_from(val: usize) -> Result<Position, Self::Error> {
        if val < 1 || val > 9 {
            return Err(PositionError::ValueError(val));
        }

        Ok(Position(val))
    }
}

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

impl std::ops::Index<Position> for Board {
    type Output = char;

    fn index(&self, index: Position) -> &Self::Output {
        &self.the_board[*index - 1]
    }
}

// Should be removed (Invariant)
impl std::ops::IndexMut<Position> for Board {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.the_board[*index - 1]
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
    pub(crate) fn get_cell(&self, cell_id: Position) -> char {
        self.the_board[*cell_id - 1]
    }

    /// Set the value stored in a selected Cell.
    ///
    /// Args:
    ///     cell_id: numeric id representing the desired cell
    ///     new_value: replacement `CellValue`
    ///
    pub fn set_cell(&mut self, cell_id: Position, new_value: Symbol) {
        self.the_board[*cell_id - 1] = new_value.as_char();
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
        !self.the_board.iter().any(|cell: &char| cell.is_numeric())
    }

    pub fn cell_is_empty(&self, position: Position) -> bool {
        let symbol = self.get_cell(position);

        // !VALID_SYMBOLS.iter().contains(&symbol)
        symbol.is_numeric()
    }

    pub fn cell_is_not_empty(&self, position: Position) -> bool {
        !self.cell_is_empty(position)
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
