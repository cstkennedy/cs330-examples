import pytest
from hamcrest import (
    assert_that,
    contains_exactly,
    equal_to,
    has_string,
    is_,
    is_not,
)

from tictactoe.board import Board

EXPECTED_EMPTY_STR = "1|2|3\n4|5|6\n7|8|9"


@pytest.fixture
def a_board():
    yield Board()


def test_default_constructor(a_board):
    for expected_char in range(1, 10):
        assert_that(a_board.get_cell(expected_char), is_(str(expected_char)))

    assert_that(a_board, has_string(EXPECTED_EMPTY_STR))
    assert_that(not a_board.is_full())

    assert_that(
        a_board.rows(),
        contains_exactly(
            list("123"),
            list("456"),
            list("789"),
        ),
    )

    assert_that(
        a_board.columns(),
        contains_exactly(
            list("147"),
            list("258"),
            list("369"),
        ),
    )

    assert_that(
        a_board.diagonals(),
        contains_exactly(
            list("159"),
            list("357"),
        ),
    )


def test_set_cell(a_board):
    a_board.set_cell(1, "X")
    a_board.set_cell(9, "O")

    assert_that(a_board.get_cell(1), is_("X"))
    assert_that(a_board.get_cell(9), is_("O"))

    assert_that(str(a_board), is_not(equal_to(EXPECTED_EMPTY_STR)))
    assert_that(a_board, has_string("X|2|3\n4|5|6\n7|8|O"))

    assert_that(
        a_board.rows(),
        contains_exactly(
            list("X23"),
            list("456"),
            list("78O"),
        ),
    )

    assert_that(
        a_board.columns(),
        contains_exactly(
            list("X47"),
            list("258"),
            list("36O"),
        ),
    )

    assert_that(
        a_board.diagonals(),
        contains_exactly(
            list("X5O"),
            list("357"),
        ),
    )

    assert_that(not a_board.is_full())


@pytest.mark.parametrize("cell_id", [0, -1, 10, 11])
def test_get_cell_bounds_check(a_board, cell_id):
    with pytest.raises(IndexError):
        a_board.get_cell(cell_id)


@pytest.mark.parametrize("cell_id", [0, -1, 10, 11])
def test_set_cell_bounds_check(a_board, cell_id):
    with pytest.raises(IndexError):
        a_board.set_cell(cell_id, "X")


@pytest.mark.parametrize("symbol", ["x", "o", "?"])
def test_set_cell_symbol_check(a_board, symbol):
    with pytest.raises(ValueError):
        a_board.set_cell(1, symbol)
