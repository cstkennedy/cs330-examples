use itertools::Itertools;

use crate::board::Board;

#[derive(Debug, PartialEq)]
pub enum TurnResult {
    Win,
    Stalemate,
    NotOverYet,
    Forfeit,
}

#[derive(Debug, Default, PartialEq)]
pub struct Referee;

impl Referee {
    pub fn check_for_win(board: &Board) -> bool {
        board
            .rows()
            .iter()
            .chain(board.columns().iter())
            .chain(board.diagonals().iter())
            .filter(|&cell_triple| cell_triple.iter().all_equal())
            .count()
            != 0
    }

    // TODO: add a test
    pub fn determine_turn_result(board: &Board) -> TurnResult {
        if Self::check_for_win(board) {
            return TurnResult::Win;
        }

        if board.is_full() {
            return TurnResult::Stalemate;
        }

        TurnResult::NotOverYet
    }
}
