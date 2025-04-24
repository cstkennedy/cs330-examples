import pytest
from hamcrest import assert_that, equal_to, instance_of, is_

import tictactoe.factories as factories
from tictactoe import Board, Game, Player
from tictactoe.board import NullRender
from tictactoe.game import CompletedGame, TurnResult


def test_constructor():
    tom = Player(name="Tom", strategy=None, preferred_renderer=NullRender)
    a_cylon = Player(strategy=None, preferred_renderer=NullRender)
    a_game = Game(player1=tom, player2=a_cylon)

    assert_that(a_game.player1, equal_to(tom))
    assert_that(a_game.player2, equal_to(a_cylon))

    # Can not test without Board.equals method
    empty_board = Board()
    assert_that(a_game.board, equal_to(empty_board))


def test_game_start_with_no_players():
    pytest.skip("Test no longer makes sense")


def test_player_turn():
    Game.use_defaults()
    game = Game(
        player1=(
            Player.create_computer(
                name=("Player 1"),
                strategy=factories.MoveStrategyFactory.create(
                    "SetMoves", moves=[5, 3, 4, 9, 8]
                ),
            )
        ),
        player2=(
            Player.create_computer(
                name=("Player 2"),
                strategy=factories.MoveStrategyFactory.create(
                    "SetMoves", moves=[1, 7, 6, 2]
                ),
            )
        ),
    )

    turn_result = game._player_turn(game.player1, "X")
    assert_that(game.board.get_cell(5), is_(equal_to("X")))
    assert_that(turn_result, is_(equal_to(TurnResult.NOT_OVER_YET)))

    turn_result = game._player_turn(game.player2, "O")
    assert_that(game.board.get_cell(1), is_(equal_to("O")))
    assert_that(turn_result, is_(equal_to(TurnResult.NOT_OVER_YET)))


def test_play_match_to_stalemate():
    Game.use_defaults()
    game = Game(
        player1=(
            Player.create_computer(
                name=("Player 1"),
                strategy=factories.MoveStrategyFactory.create(
                    "SetMoves", moves=[5, 3, 4, 9, 8]
                ),
            )
        ),
        player2=(
            Player.create_computer(
                name=("Player 2"),
                strategy=factories.MoveStrategyFactory.create(
                    "SetMoves", moves=[1, 7, 6, 2]
                ),
            )
        ),
    ).play_match()

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

    assert_that(game.board, equal_to(expected_board))
    assert_that(game, is_(instance_of(CompletedGame.Stalemate)))


def test_play_match_to_win_player_1():
    Game.use_defaults()
    game = Game(
        player1=(
            Player.create_computer(
                name=("Player 1"),
                strategy=factories.MoveStrategyFactory.create(
                    "SetMoves", moves=[1, 3, 2]
                ),
            )
        ),
        player2=(
            Player.create_computer(
                name=("Player 2"),
                strategy=factories.MoveStrategyFactory.create(
                    "SetMoves", moves=[4, 6, 5]
                ),
            )
        ),
    ).play_match()

    expected_board = Board()
    expected_board.set_cell(1, "X")
    expected_board.set_cell(2, "X")
    expected_board.set_cell(3, "X")
    expected_board.set_cell(4, "O")
    #  expected_board.set_cell(5, "O")
    expected_board.set_cell(6, "O")

    assert_that(game.board, equal_to(expected_board))

    assert_that(game, is_(instance_of(CompletedGame.Win)))

    assert_that(game.winner.name, is_(equal_to("Player 1")))
    assert_that(game.loser.name, is_(equal_to("Player 2")))


def test_play_match_to_win_player_2():
    Game.use_defaults()
    game = Game(
        player1=(
            Player.create_computer(
                name=("Player 1"),
                strategy=factories.MoveStrategyFactory.create(
                    "SetMoves", moves=[1, 3, 7]
                ),
            )
        ),
        player2=(
            Player.create_computer(
                name=("Player 2"),
                strategy=factories.MoveStrategyFactory.create(
                    "SetMoves", moves=[5, 2, 8]
                ),
            )
        ),
    ).play_match()

    expected_board = Board()
    expected_board.set_cell(1, "X")
    expected_board.set_cell(3, "X")
    expected_board.set_cell(7, "X")
    expected_board.set_cell(2, "O")
    expected_board.set_cell(5, "O")
    expected_board.set_cell(8, "O")

    assert_that(game.board, equal_to(expected_board))

    assert_that(game, is_(instance_of(CompletedGame.Win)))

    assert_that(game.winner.name, is_(equal_to("Player 2")))
    assert_that(game.loser.name, is_(equal_to("Player 1")))
