#[cfg(test)]
use hamcrest2::prelude::*;
use rstest::*;

use tictactoe::board::{Board, Position, Symbol};
use tictactoe::error::PositionError;
use tictactoe::referee::Referee;

#[rstest]
pub fn test_check_for_win_empty_board() -> Result<(), PositionError> {
    let board = Board::new();

    assert_that!(Referee::check_for_win(&board), is(false));

    for idx in 1..10 {
        let idx = Position::try_from(idx)?;
        assert_that!(board.cell_is_empty(idx), is(true));
    }

    assert_that!(Referee::check_for_win(&board), is(false));

    Ok(())
}

#[rstest]
pub fn test_check_for_horizontal_win() -> Result<(), PositionError> {
    let mut h_board = Board::new();
    h_board.set_cell(Position::try_from(4)?, Symbol::X);
    h_board.set_cell(Position::try_from(5)?, Symbol::X);
    h_board.set_cell(Position::try_from(6)?, Symbol::X);

    assert_that!(h_board.cell_is_empty(Position::try_from(4)?), is(false));
    assert_that!(h_board.cell_is_empty(Position::try_from(5)?), is(false));
    assert_that!(h_board.cell_is_empty(Position::try_from(6)?), is(false));

    assert_that!(Referee::check_for_win(&h_board), is(true));

    Ok(())
}

#[rstest]
pub fn test_check_for_vertical_win() -> Result<(), PositionError> {
    let mut v_board = Board::new();
    let _ = v_board.set_cell(Position::try_from(2)?, Symbol::O);
    let _ = v_board.set_cell(Position::try_from(5)?, Symbol::O);
    let _ = v_board.set_cell(Position::try_from(8)?, Symbol::O);

    assert_that!(v_board.cell_is_empty(Position::try_from(2)?), is(false));
    assert_that!(v_board.cell_is_empty(Position::try_from(5)?), is(false));
    assert_that!(v_board.cell_is_empty(Position::try_from(8)?), is(false));

    assert_that!(Referee::check_for_win(&v_board), is(true));

    Ok(())
}

#[rstest]
pub fn test_check_for_diagonal_win() -> Result<(), PositionError> {
    let mut d_board = Board::new();
    let _ = d_board.set_cell(Position::try_from(3)?, Symbol::O);
    let _ = d_board.set_cell(Position::try_from(5)?, Symbol::O);
    let _ = d_board.set_cell(Position::try_from(7)?, Symbol::O);

    assert_that!(d_board.cell_is_empty(Position::try_from(3)?), is(false));
    assert_that!(d_board.cell_is_empty(Position::try_from(5)?), is(false));
    assert_that!(d_board.cell_is_empty(Position::try_from(7)?), is(false));

    assert_that!(Referee::check_for_win(&d_board), is(true));

    Ok(())
}
