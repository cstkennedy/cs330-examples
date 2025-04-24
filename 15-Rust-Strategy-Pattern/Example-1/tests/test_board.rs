#[cfg(test)]
use hamcrest2::prelude::*;
use rstest::*;

use tictactoe::board::Board;

const EXPECTED_EMPTY_STR: &str = "1|2|3\n4|5|6\n7|8|9";

#[fixture]
fn a_board() -> Board {
    Board::new()
}

#[rstest]
pub fn test_default_constructor(a_board: Board) {
    for position in 1..=9 {
        let expected_char = char::from_digit(position as u32, 10).unwrap();
        assert_that!(a_board.get_cell(position), is(ok()));
        assert_that!(
            a_board.get_cell(position).unwrap(),
            is(equal_to(expected_char))
        );
    }

    assert_that!(a_board.to_string(), is(equal_to(EXPECTED_EMPTY_STR)));
    assert_that!(a_board.is_full(), is(not(true)));

    let rows = a_board.rows();
    assert_that!(
        &rows,
        contains(vec![['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9'],])
    );

    let cols = a_board.columns();
    assert_that!(
        &cols,
        contains(vec![['1', '4', '7'], ['2', '5', '8'], ['3', '6', '9'],])
    );

    let diagonals = a_board.diagonals();
    assert_that!(
        &diagonals,
        contains(vec![['1', '5', '9'], ['3', '5', '7'],])
    );
}

#[rstest]
pub fn test_set_cell(mut a_board: Board) {
    assert_that!(a_board.set_cell(1, 'X'), is(ok()));
    assert_that!(a_board.set_cell(9, 'O'), is(ok()));

    assert_that!(a_board.get_cell(1), is(equal_to(Ok('X'))));
    assert_that!(a_board.get_cell(9), is(equal_to(Ok('O'))));

    assert_that!(a_board.to_string(), is_not(equal_to(EXPECTED_EMPTY_STR)));
    assert_that!(a_board.to_string(), is(equal_to("X|2|3\n4|5|6\n7|8|O")));

    let rows = a_board.rows();
    assert_that!(
        &rows,
        contains(vec![['X', '2', '3'], ['4', '5', '6'], ['7', '8', 'O'],])
    );

    let cols = a_board.columns();
    assert_that!(
        &cols,
        contains(vec![['X', '4', '7'], ['2', '5', '8'], ['3', '6', 'O'],])
    );

    let diagonals = a_board.diagonals();
    assert_that!(
        &diagonals,
        contains(vec![['X', '5', 'O'], ['3', '5', '7'],])
    );

    assert_that!(a_board.is_full(), is(not(true)));
}

#[rstest]
#[case(0)]
#[case(10)]
#[case(11)]
pub fn test_get_cell_bounds_check(#[case] cell_id: usize) {
    let a_board = Board::new();
    assert_that!(a_board.get_cell(cell_id), is(err()));
}

#[rstest]
#[case(0)]
#[case(10)]
#[case(11)]
pub fn test_set_cell_bounds_check(#[case] cell_id: usize) {
    let mut a_board = Board::new();
    assert_that!(a_board.set_cell(cell_id, 'X'), is(err()));
    assert_that!(a_board.set_cell(cell_id, 'O'), is(err()));
}

#[rstest]
#[case('?')]
#[case('+')]
#[case('x')]
#[case('o')]
pub fn test_set_cell_symbol_check(#[case] symbol: char) {
    let mut a_board = Board::new();
    assert_that!(a_board.set_cell(1, symbol), is(err()));
    assert_that!(a_board.set_cell(2, symbol), is(err()));
    assert_that!(a_board.set_cell(3, symbol), is(err()));
    assert_that!(a_board.set_cell(4, symbol), is(err()));
    assert_that!(a_board.set_cell(5, symbol), is(err()));
    assert_that!(a_board.set_cell(6, symbol), is(err()));
    assert_that!(a_board.set_cell(7, symbol), is(err()));
    assert_that!(a_board.set_cell(8, symbol), is(err()));
    assert_that!(a_board.set_cell(9, symbol), is(err()));
}
