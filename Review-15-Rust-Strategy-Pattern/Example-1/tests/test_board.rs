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

    // assert_that!(a_board, has_string(EXPECTED_EMPTY_STR))
    assert_that!(a_board.is_full(), is(not(true)));

    assert_that!(
        &a_board.rows().iter().collect::<Vec<&[char; 3]>>(),
        contains(vec![
            &['1', '2', '3'],
            &['4', '5', '6'],
            &['7', '8', '9'],
        ])
    );

    /*
    assert_that!(
        a_board.columns(),
        contains_exactly(
            list("147"),
            list("258"),
            list("369"),
        ),
    )

    assert_that!(
        a_board.diagonals(),
        contains_exactly(
            list("159"),
            list("357"),
        ),
    )
    */
}

/*

def test_set_cell(a_board):
    a_board.set_cell(1, "X")
    a_board.set_cell(9, "O")

    assert_that!(a_board.get_cell(1), is_("X"))
    assert_that!(a_board.get_cell(9), is_("O"))

    assert_that!(str(a_board), is_not(equal_to(EXPECTED_EMPTY_STR)))
    assert_that!(a_board, has_string("X|2|3\n4|5|6\n7|8|O"))

    assert_that!(
        a_board.rows(),
        contains_exactly(
            list("X23"),
            list("456"),
            list("78O"),
        ),
    )

    assert_that!(
        a_board.columns(),
        contains_exactly(
            list("X47"),
            list("258"),
            list("36O"),
        ),
    )

    assert_that!(
        a_board.diagonals(),
        contains_exactly(
            list("X5O"),
            list("357"),
        ),
    )

    assert_that!(not a_board.is_full())
*/

#[rstest]
#[case(0)]
#[case(10)]
#[case(11)]
pub fn test_get_cell_bounds_check(#[case] cell_id: usize) {
    let a_board = Board::new();
    assert_that!(a_board.get_cell(cell_id), is(err()));
}

/*
@pytest.mark.parametrize("cell_id", [0, -1, 10, 11])
def test_set_cell_bounds_check(a_board, cell_id):
    with pytest.raises(IndexError):
        a_board.set_cell(cell_id, "X")


@pytest.mark.parametrize("symbol", ["x", "o", "?"])
def test_set_cell_symbol_check(a_board, symbol):
    with pytest.raises(ValueError):
        a_board.set_cell(1, symbol)
*/
