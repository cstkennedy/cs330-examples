import pytest
from hamcrest import assert_that, equal_to, is_, is_not, none

from tictactoe import Board, Game, GameState, GameStateError, Player
from tictactoe.board import NullRender, RenderBoardToScreen
from tictactoe.builders import GameBuilder, PlayerBuilder


def test_constructor():
    tom = Player(name="Tom", strategy=None, preferred_renderer=NullRender)
    a_cylon = Player(strategy=None, preferred_renderer=NullRender)
    a_game = Game(tom, a_cylon)

    assert_that(a_game.get_player1(), equal_to(tom))
    assert_that(a_game.get_player2(), equal_to(a_cylon))

    assert_that(a_game.is_over(), is_(False))

    assert_that(a_game.get_winner(), is_(none()))
    assert_that(a_game.get_loser(), is_(none()))

    # Can not test without Board.equals method
    empty_board = Board()
    assert_that(a_game.get_board(), equal_to(empty_board))


def test_game_start_with_no_players():
    game = Game(None, None)

    assert_that(game.get_player1(), is_(none()))
    assert_that(game.get_player2(), is_(none()))

    assert_that(game.ready_to_start(), is_(False))
    assert_that(game.not_ready_to_start(), is_(True))

    assert_that(game.current_state(), is_(GameState.NOT_STARTED))

    with pytest.raises(GameStateError):
        game.play_match()


def test_player_turn():
    game = Game(
        PlayerBuilder.builder()
        .with_name("Player 1")
        .with_strategy(name="SetMoves", moves=[5, 3, 4, 9, 8])
        .build(),
        PlayerBuilder.builder()
        .with_name("Player 2")
        .with_strategy(name="SetMoves", moves=[1, 7, 6, 2])
        .build(),
    )

    board = game.get_board()

    game.player_turn(game.get_player1(), "X")

    assert_that(game.current_state(), is_(equal_to(GameState.IN_PROGRESS)))
    assert_that(board.get_cell(5), is_(equal_to("X")))

    game.player_turn(game.get_player2(), "O")

    assert_that(game.current_state(), is_(equal_to(GameState.IN_PROGRESS)))
    assert_that(board.get_cell(1), is_(equal_to("O")))

    assert_that(game.is_over(), is_(False))
    assert_that(game.is_not_over(), is_(True))


def test_play_match_to_stalemate():
    game = (
        GameBuilder.builder()
        .add_player(name="Player 1", strategy="SetMoves", moves=[5, 3, 4, 9, 8])
        .add_player(name="Player 2", strategy="SetMoves", moves=[1, 7, 6, 2])
        .build()
    )

    game.play_match()

    assert_that(game.get_player1().name, is_(equal_to("Player 1")))
    assert_that(game.get_player2().name, is_(equal_to("Player 2")))

    assert_that(game.is_over(), is_(True))
    assert_that(game.is_not_over(), is_(False))

    expected_board = Board()
    expected_board.set_cell(5, "X")
    expected_board.set_cell(1, "O")
    expected_board.set_cell(3, "X")
    expected_board.set_cell(7, "O")
    expected_board.set_cell(4, "X")
    expected_board.set_cell(6, "O")
    expected_board.set_cell(9, "X")
    expected_board.set_cell(2, "O")
    expected_board.set_cell(8, "X")

    assert_that(game.get_board(), equal_to(expected_board))

    assert_that(game.get_winner(), is_(none()))
    assert_that(game.get_loser(), is_(none()))

    assert_that(game.ended_with_win(), is_(False))
    assert_that(game.ended_with_loss(), is_(False))
    assert_that(game.ended_with_stalemate(), is_(True))

    assert_that(game.current_state(), is_(GameState.OVER_WITH_STALEMATE))


def test_play_match_to_win_player_1():
    game = (
        GameBuilder.builder()
        .add_player(name="Player 1", strategy="SetMoves", moves=[1, 3, 2])
        .add_player(name="Player 2", strategy="SetMoves", moves=[4, 6, 5])
        .build()
    )

    game.play_match()

    assert_that(game.get_player1().name, is_(equal_to("Player 1")))
    assert_that(game.get_player2().name, is_(equal_to("Player 2")))

    assert_that(game.is_over(), is_(True))
    assert_that(game.is_not_over(), is_(False))

    expected_board = Board()
    expected_board.set_cell(1, "X")
    expected_board.set_cell(2, "X")
    expected_board.set_cell(3, "X")
    expected_board.set_cell(4, "O")
    #  expected_board.set_cell(5, "O")
    expected_board.set_cell(6, "O")

    assert_that(game.get_board(), equal_to(expected_board))

    assert_that(game.get_winner(), is_not(none()))
    assert_that(game.get_loser(), is_not(none()))

    assert_that(game.get_winner(), is_(equal_to(game.get_player1())))
    assert_that(game.get_loser(), is_(equal_to(game.get_player2())))

    assert_that(game.ended_with_win(), is_(True))
    assert_that(game.ended_with_loss(), is_(True))
    assert_that(game.ended_with_stalemate(), is_(False))

    assert_that(game.current_state(), is_(GameState.OVER_WITH_WIN))


def test_play_match_to_win_player_2():
    game = (
        GameBuilder.builder()
        .add_player(name="Player 1", strategy="SetMoves", moves=[1, 3, 7])
        .add_player(name="Player 2", strategy="SetMoves", moves=[5, 2, 8])
        .build()
    )

    game.play_match()

    assert_that(game.get_player1().name, is_(equal_to("Player 1")))
    assert_that(game.get_player2().name, is_(equal_to("Player 2")))

    assert_that(game.is_over(), is_(True))
    assert_that(game.is_not_over(), is_(False))

    expected_board = Board()
    expected_board.set_cell(1, "X")
    expected_board.set_cell(3, "X")
    expected_board.set_cell(7, "X")
    expected_board.set_cell(2, "O")
    expected_board.set_cell(5, "O")
    expected_board.set_cell(8, "O")

    assert_that(game.get_board(), equal_to(expected_board))

    assert_that(game.get_winner(), is_not(none()))
    assert_that(game.get_loser(), is_not(none()))

    assert_that(game.get_winner(), is_(equal_to(game.get_player2())))
    assert_that(game.get_loser(), is_(equal_to(game.get_player1())))

    assert_that(game.ended_with_win(), is_(True))
    assert_that(game.ended_with_loss(), is_(True))
    assert_that(game.ended_with_stalemate(), is_(False))

    assert_that(game.current_state(), is_(GameState.OVER_WITH_WIN))
