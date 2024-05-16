import copy

import pytest
from hamcrest import assert_that, is_

from examples import Board, Game, Player, Referee


@pytest.fixture
def empty_board_and_ref():
    empty_board = Board()

    a_referee = Referee(empty_board)

    yield empty_board, a_referee


def test_constructor(empty_board_and_ref):
    _, a_referee = empty_board_and_ref

    assert_that(a_referee.check_for_win(), is_(0))

    for i in range(1, 10):
        assert_that(a_referee.selected_cell_is_empty(i), is_(True))


def test_check_for_horizontal_win():
    h_board = Board()

    h_board.set_cell(4, "X")
    h_board.set_cell(5, "X")
    h_board.set_cell(6, "X")

    h_referee = Referee(h_board)

    assert_that(h_referee.selected_cell_is_empty(4), is_(False))
    assert_that(h_referee.selected_cell_is_empty(5), is_(False))
    assert_that(h_referee.selected_cell_is_empty(6), is_(False))

    assert_that(h_referee.check_for_win(), is_(1))


def test_check_for_vertical_win():
    v_board = Board()

    v_board.set_cell(2, "O")
    v_board.set_cell(5, "O")
    v_board.set_cell(8, "O")

    v_referee = Referee(v_board)

    assert_that(v_referee.selected_cell_is_empty(2), is_(False))
    assert_that(v_referee.selected_cell_is_empty(5), is_(False))
    assert_that(v_referee.selected_cell_is_empty(8), is_(False))

    assert_that(v_referee.check_for_win(), is_(2))


def test_check_for_diagonal_win():
    d_board = Board()

    d_board.set_cell(3, "O")
    d_board.set_cell(5, "O")
    d_board.set_cell(7, "O")

    d_referee = Referee(d_board)

    assert_that(d_referee.selected_cell_is_empty(3), is_(False))
    assert_that(d_referee.selected_cell_is_empty(5), is_(False))
    assert_that(d_referee.selected_cell_is_empty(7), is_(False))

    assert_that(d_referee.check_for_win(), is_(2))
