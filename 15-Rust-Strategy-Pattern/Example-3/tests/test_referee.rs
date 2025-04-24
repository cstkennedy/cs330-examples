#[cfg(test)]
use hamcrest2::prelude::*;
use rstest::*;

use tictactoe::board::Board;
use tictactoe::referee::Referee;

#[rstest]
pub fn test_check_for_win_empty_board() {
    let board = Board::new();

    assert_that!(Referee::check_for_win(&board), is(false));

    for idx in 1..10 {
        assert_that!(Referee::selected_cell_is_empty(idx, &board), is(true));
    }

    assert_that!(Referee::check_for_win(&board), is(false));
}

#[rstest]
pub fn test_check_for_horizontal_win() {
    let mut h_board = Board::new();
    let _ = h_board.set_cell(4, 'X');
    let _ = h_board.set_cell(5, 'X');
    let _ = h_board.set_cell(6, 'X');

    assert_that!(Referee::selected_cell_is_empty(4, &h_board), is(false));
    assert_that!(Referee::selected_cell_is_empty(5, &h_board), is(false));
    assert_that!(Referee::selected_cell_is_empty(6, &h_board), is(false));

    assert_that!(Referee::check_for_win(&h_board), is(true));
}

#[rstest]
pub fn test_check_for_vertical_win() {
    let mut v_board = Board::new();
    let _ = v_board.set_cell(2, 'O');
    let _ = v_board.set_cell(5, 'O');
    let _ = v_board.set_cell(8, 'O');

    assert_that!(Referee::selected_cell_is_empty(2, &v_board), is(false));
    assert_that!(Referee::selected_cell_is_empty(5, &v_board), is(false));
    assert_that!(Referee::selected_cell_is_empty(8, &v_board), is(false));

    assert_that!(Referee::check_for_win(&v_board), is(true));
}

#[rstest]
pub fn test_check_for_diagonal_win() {
    let mut d_board = Board::new();
    let _ = d_board.set_cell(3, 'O');
    let _ = d_board.set_cell(5, 'O');
    let _ = d_board.set_cell(7, 'O');

    assert_that!(Referee::selected_cell_is_empty(3, &d_board), is(false));
    assert_that!(Referee::selected_cell_is_empty(5, &d_board), is(false));
    assert_that!(Referee::selected_cell_is_empty(7, &d_board), is(false));

    assert_that!(Referee::check_for_win(&d_board), is(true));
}
