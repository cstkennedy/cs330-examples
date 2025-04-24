import pytest
from hamcrest import assert_that, is_, none

from tictactoe import Board, Referee


@pytest.fixture
def empty_board_and_ref():
    empty_board = Board()

    a_referee = Referee(empty_board)

    yield empty_board, a_referee


def test_constructor(empty_board_and_ref):
    _, a_referee = empty_board_and_ref

    assert_that(a_referee.check_for_win(), is_(none()))

    for idx in range(1, 10):
        assert_that(a_referee.selected_cell_is_empty(idx), is_(True))

    assert_that(a_referee.check_for_win(), is_(none()))


def test_check_for_horizontal_win(empty_board_and_ref):
    h_board, h_referee = empty_board_and_ref
    h_board.set_cell(4, "X")
    h_board.set_cell(5, "X")
    h_board.set_cell(6, "X")

    assert_that(h_referee.selected_cell_is_empty(4), is_(False))
    assert_that(h_referee.selected_cell_is_empty(5), is_(False))
    assert_that(h_referee.selected_cell_is_empty(6), is_(False))

    assert_that(h_referee.check_for_win(), is_("X"))


def test_check_for_vertical_win(empty_board_and_ref):
    v_board, v_referee = empty_board_and_ref
    v_board.set_cell(2, "O")
    v_board.set_cell(5, "O")
    v_board.set_cell(8, "O")

    assert_that(v_referee.selected_cell_is_empty(2), is_(False))
    assert_that(v_referee.selected_cell_is_empty(5), is_(False))
    assert_that(v_referee.selected_cell_is_empty(8), is_(False))

    assert_that(v_referee.check_for_win(), is_("O"))


def test_check_for_diagonal_win(empty_board_and_ref):
    d_board, d_referee = empty_board_and_ref
    d_board.set_cell(3, "O")
    d_board.set_cell(5, "O")
    d_board.set_cell(7, "O")

    assert_that(d_referee.selected_cell_is_empty(3), is_(False))
    assert_that(d_referee.selected_cell_is_empty(5), is_(False))
    assert_that(d_referee.selected_cell_is_empty(7), is_(False))

    assert_that(d_referee.check_for_win(), is_("O"))
