import pytest
from hamcrest import assert_that, contains_exactly, equal_to, has_string, is_, is_not

from examples import Board

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
        a_board.get_3_cells(1, 2, 3),
        contains_exactly((0, "1"), (1, "2"), (2, "3")),
    )


def test_set_cell(a_board):
    a_board.set_cell(1, "X")
    a_board.set_cell(9, "O")

    assert_that(a_board.get_cell(1), is_("X"))
    assert_that(a_board.get_cell(9), is_("O"))

    assert_that(
        a_board.get_3_cells(1, 5, 9),
        contains_exactly((0, "X"), (4, "5"), (8, "O")),
    )

    assert_that(str(a_board), is_not(equal_to(EXPECTED_EMPTY_STR)))
    assert_that(a_board, has_string("X|2|3\n4|5|6\n7|8|O"))

    assert_that(not a_board.is_full())


@pytest.mark.parametrize("cell_id", [0, -1, 10, 11])
def test_get_cell_bounds_check(a_board, cell_id):
    with pytest.raises(AssertionError):
        a_board.get_cell(cell_id)


@pytest.mark.parametrize("cell_id", [0, -1, 10, 11])
def test_get_3_cells_bounds_check(a_board, cell_id):
    with pytest.raises(IndexError):
        a_board.get_3_cells(cell_id, cell_id, cell_id)

    with pytest.raises(IndexError):
        a_board.get_3_cells(cell_id, 1, 2)

    with pytest.raises(IndexError):
        a_board.get_3_cells(1, cell_id, 2)

    with pytest.raises(IndexError):
        a_board.get_3_cells(1, 2, cell_id)

    with pytest.raises(IndexError):
        a_board.get_3_cells(cell_id, cell_id, 3)

    with pytest.raises(IndexError):
        a_board.get_3_cells(cell_id, 2, cell_id)

    with pytest.raises(IndexError):
        a_board.get_3_cells(1, cell_id, cell_id)

    with pytest.raises(IndexError):
        a_board.get_3_cells(cell_id, cell_id, cell_id)


@pytest.mark.parametrize("cell_id", [0, -1, 10, 11])
def test_set_cell_bounds_check(a_board, cell_id):
    with pytest.raises(AssertionError):
        a_board.set_cell(cell_id, "X")


@pytest.mark.parametrize("symbol", ["x", "o", "?"])
def test_set_cell_symbol_check(a_board, symbol):
    with pytest.raises(ValueError):
        a_board.set_cell(1, symbol)
