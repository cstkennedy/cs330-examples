#[cfg(test)]
use hamcrest2::prelude::*;
use rstest::*;

use tictactoe::board::{Board, Position, Symbol};
use tictactoe::referee::Referee;

#[rstest]
pub fn test_check_for_win_empty_board() {
    let board = Board::new();

    assert_that!(Referee::check_for_win(&board), is(false));

    for idx in 1..10 {
        let idx = Position::try_from(idx).unwrap();
        assert_that!(Referee::selected_cell_is_empty(idx, &board), is(true));
    }

    assert_that!(Referee::check_for_win(&board), is(false));
}

#[rstest]
pub fn test_check_for_horizontal_win() {
    let mut h_board = Board::new();
    h_board.set_cell(Position::try_from(4).unwrap(), Symbol::X);
    h_board.set_cell(Position::try_from(5).unwrap(), Symbol::X);
    h_board.set_cell(Position::try_from(6).unwrap(), Symbol::X);

    assert_that!(
        Referee::selected_cell_is_empty(Position::try_from(4).unwrap(), &h_board),
        is(false)
    );
    assert_that!(
        Referee::selected_cell_is_empty(Position::try_from(5).unwrap(), &h_board),
        is(false)
    );
    assert_that!(
        Referee::selected_cell_is_empty(Position::try_from(6).unwrap(), &h_board),
        is(false)
    );

    assert_that!(Referee::check_for_win(&h_board), is(true));
}

#[rstest]
pub fn test_check_for_vertical_win() {
    let mut v_board = Board::new();
    let _ = v_board.set_cell(Position::try_from(2).unwrap(), Symbol::O);
    let _ = v_board.set_cell(Position::try_from(5).unwrap(), Symbol::O);
    let _ = v_board.set_cell(Position::try_from(8).unwrap(), Symbol::O);

    assert_that!(
        Referee::selected_cell_is_empty(Position::try_from(2).unwrap(), &v_board),
        is(false)
    );
    assert_that!(
        Referee::selected_cell_is_empty(Position::try_from(5).unwrap(), &v_board),
        is(false)
    );
    assert_that!(
        Referee::selected_cell_is_empty(Position::try_from(8).unwrap(), &v_board),
        is(false)
    );

    assert_that!(Referee::check_for_win(&v_board), is(true));
}

#[rstest]
pub fn test_check_for_diagonal_win() {
    let mut d_board = Board::new();
    let _ = d_board.set_cell(Position::try_from(3).unwrap(), Symbol::O);
    let _ = d_board.set_cell(Position::try_from(5).unwrap(), Symbol::O);
    let _ = d_board.set_cell(Position::try_from(7).unwrap(), Symbol::O);

    assert_that!(
        Referee::selected_cell_is_empty(Position::try_from(3).unwrap(), &d_board),
        is(false)
    );
    assert_that!(
        Referee::selected_cell_is_empty(Position::try_from(5).unwrap(), &d_board),
        is(false)
    );
    assert_that!(
        Referee::selected_cell_is_empty(Position::try_from(7).unwrap(), &d_board),
        is(false)
    );

    assert_that!(Referee::check_for_win(&d_board), is(true));
}
