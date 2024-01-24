import copy

import pytest
from hamcrest import *

from examples import Board, Game, Player, Referee


def test_constructor():
    tom = Player("Tom")
    a_cylon = Player()

    empty_board = Board()

    a_game = Game(tom, a_cylon)

    assert_that(a_game.get_player1(), equal_to(tom))
    assert_that(a_game.get_player2(), equal_to(a_cylon))

    assert_that(a_game.get_player1().get_symbol(), is_("X"))
    assert_that(a_game.get_player2().get_symbol(), is_("O"))

    assert_that(a_game.is_over(), is_(False))

    assert_that(a_game.get_winner(), is_(none()))
    assert_that(a_game.get_loser(), is_(none()))

    # Can not test without Board.equals method
    assert_that(a_game.get_board(), equal_to(empty_board))


@pytest.mark.skip("can not test")
def test_play_round():
    # Can not test due to hardcoded System.in use in Player.next_move
    pass
