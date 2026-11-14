#[cfg(test)]
use hamcrest2::prelude::*;
use rstest::*;

use tictactoe::board::{Board, Position, Symbol};

const EXPECTED_EMPTY_STR: &str = "1|2|3\n4|5|6\n7|8|9";

#[fixture]
fn a_board() -> Board {
    Board::new()
}

#[rstest]
pub fn test_default_constructor(a_board: Board) {
    for position in 1..=9 {
        let position = Position::try_from(position).unwrap();

        let expected_char = char::from_digit(*position as u32, 10).unwrap();
        assert_that!(a_board[position], is(equal_to(expected_char)));
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
    a_board.set_cell(Position::try_from(1).unwrap(), Symbol::X);
    a_board.set_cell(Position::try_from(9).unwrap(), Symbol::O);

    assert_that!(a_board[Position::try_from(1).unwrap()], is(equal_to('X')));
    assert_that!(a_board[Position::try_from(9).unwrap()], is(equal_to('O')));

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

// Added when debuging test_game::test_stalemate
#[rstest]
pub fn test_is_full(mut a_board: Board) {
    a_board.set_cell(Position::try_from(1).unwrap(), Symbol::X);
    a_board.set_cell(Position::try_from(2).unwrap(), Symbol::O);
    a_board.set_cell(Position::try_from(3).unwrap(), Symbol::X);
    a_board.set_cell(Position::try_from(4).unwrap(), Symbol::O);
    a_board.set_cell(Position::try_from(5).unwrap(), Symbol::X);
    a_board.set_cell(Position::try_from(6).unwrap(), Symbol::O);
    a_board.set_cell(Position::try_from(7).unwrap(), Symbol::X);
    a_board.set_cell(Position::try_from(8).unwrap(), Symbol::O);
    a_board.set_cell(Position::try_from(9).unwrap(), Symbol::X);

    assert_that!(a_board.is_full(), is(true));
}

#[rstest]
#[case(0)]
#[case(10)]
#[case(11)]
pub fn test_position_bounds_check(#[case] cell_id: usize) {
    assert_that!(Position::try_from(cell_id), is(err()));
}
