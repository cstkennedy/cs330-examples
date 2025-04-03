from hamcrest import assert_that, is_

from tictactoe import Board, Referee


def test_empty_board():
    board = Board()

    for idx in range(1, 10):
        assert_that(Referee.selected_cell_is_empty(board, idx), is_(True))

    assert_that(Referee.check_for_win(board), is_(False))


def test_check_for_horizontal_win():
    h_board = Board()
    h_board.set_cell(4, "X")
    h_board.set_cell(5, "X")
    h_board.set_cell(6, "X")

    assert_that(Referee.selected_cell_is_empty(h_board, 4), is_(False))
    assert_that(Referee.selected_cell_is_empty(h_board, 5), is_(False))
    assert_that(Referee.selected_cell_is_empty(h_board, 6), is_(False))

    assert_that(Referee.check_for_win(h_board), is_(True))


def test_check_for_vertical_win():
    v_board = Board()
    v_board.set_cell(2, "O")
    v_board.set_cell(5, "O")
    v_board.set_cell(8, "O")

    assert_that(Referee.selected_cell_is_empty(v_board, 2), is_(False))
    assert_that(Referee.selected_cell_is_empty(v_board, 5), is_(False))
    assert_that(Referee.selected_cell_is_empty(v_board, 8), is_(False))

    assert_that(Referee.check_for_win(v_board), is_(True))


def test_check_for_diagonal_win():
    d_board = Board()
    d_board.set_cell(3, "O")
    d_board.set_cell(5, "O")
    d_board.set_cell(7, "O")

    assert_that(Referee.selected_cell_is_empty(d_board, 3), is_(False))
    assert_that(Referee.selected_cell_is_empty(d_board, 5), is_(False))
    assert_that(Referee.selected_cell_is_empty(d_board, 7), is_(False))

    assert_that(Referee.check_for_win(d_board), is_(True))
