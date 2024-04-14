import copy

import pytest
from hamcrest import assert_that, equal_to, is_, none

from examples import Board, Game, GameState, GameStateError, Player, Referee


def test_constructor():
    empty_board = Board()

    a_game = Game()

    assert_that(a_game.get_player1(), is_(none()))
    assert_that(a_game.get_player2(), is_(none()))

    assert_that(a_game.is_over(), is_(False))

    assert_that(a_game.get_winner(), is_(none()))
    assert_that(a_game.get_loser(), is_(none()))

    # Can not test without Board.equals method
    assert_that(a_game.get_board(), equal_to(empty_board))

    assert_that(a_game.ready_to_start(), is_(False))
    assert_that(a_game.not_ready_to_start(), is_(True))

    assert_that(a_game.current_state(), is_(GameState.NOT_STARTED))


def test_constructor_with_set_players():
    tom = Player(name="Tom", strategy=None)
    a_cylon = Player(strategy=None)

    empty_board = Board()

    a_game = Game()
    a_game.set_players(tom, a_cylon)

    assert_that(a_game.get_player1(), equal_to(tom))
    assert_that(a_game.get_player2(), equal_to(a_cylon))

    assert_that(a_game.get_player1().get_symbol(), is_("X"))
    assert_that(a_game.get_player2().get_symbol(), is_("O"))

    assert_that(a_game.is_over(), is_(False))

    assert_that(a_game.get_winner(), is_(none()))
    assert_that(a_game.get_loser(), is_(none()))

    # Can not test without Board.equals method
    assert_that(a_game.get_board(), equal_to(empty_board))


def test_game_start_with_no_players():
    game = Game()

    assert_that(game.get_player1(), is_(none()))
    assert_that(game.get_player2(), is_(none()))

    assert_that(game.ready_to_start(), is_(False))
    assert_that(game.not_ready_to_start(), is_(True))

    assert_that(game.current_state(), is_(GameState.NOT_STARTED))

    with pytest.raises(GameStateError):
        game.play_match()


@pytest.mark.skip("cannot test")
def test_play_round():
    pass


@pytest.mark.skip("cannot test")
def test_play_match_to_stalemate():
    pass


@pytest.mark.skip("cannot test")
def test_play_match_to_win_player_1():
    pass


@pytest.mark.skip("cannot test")
def test_play_match_to_win_player_2():
    pass
