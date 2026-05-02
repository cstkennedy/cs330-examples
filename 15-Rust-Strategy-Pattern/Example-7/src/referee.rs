use itertools::Itertools;

use crate::board::{Board, VALID_SYMBOLS};

#[derive(Debug, Default, PartialEq)]
pub struct Referee;

impl Referee {
    pub fn check_for_win(board: &Board) -> bool {
        /*
        board
            .rows()
            .iter()
            .chain(board.columns().iter())
            .chain(board.diagonals().iter())
            .map(|&cell_triple| cell_triple.iter().all_equal())
            .contains(&true)
        */
        board
            .rows()
            .iter()
            .chain(board.columns().iter())
            .chain(board.diagonals().iter())
            .filter(|&cell_triple| cell_triple.iter().all_equal())
            .count() != 0
    }

    /// Determine whether a cell in the board has been selected
    /// by a player.
    ///
    #[deprecated]
    pub fn selected_cell_is_empty(candidate_move: usize, board: &Board) -> bool {
        board.cell_is_empty(candidate_move).unwrap_or(false)
    }
}
