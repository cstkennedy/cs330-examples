from hamcrest import *
import unittest

from examples import (Player, Referee, Board, Game)

import copy


class TestGame(unittest.TestCase):
    """
    1 - Does this piece of code perform the operations
        it was designed to perform?

    2 - Does this piece of code do something it was not
        designed to perform?

    1 Test per mutator
    """
    def setUp(self):

        self.tom = Player("Tom")
        self.a_cylon = Player()

        self.empty_board = Board()

        self.a_game = Game(self.tom, self.a_cylon)

    def test_constructor(self):

        assert_that(self.a_game.get_player1(), equal_to(self.tom))
        assert_that(self.a_game.get_player2(), equal_to(self.a_cylon))

        assert_that(self.a_game.get_player1().get_symbol(), is_('X'))
        assert_that(self.a_game.get_player2().get_symbol(), is_('O'))

        assert_that(self.a_game.is_over(), is_(False))

        assert_that(self.a_game.get_winner(), is_(none()))
        assert_that(self.a_game.get_loser(), is_(none()))

        # Can not test without Board.equals method
        assert_that(self.a_game.get_board(), equal_to(self.empty_board))

    @unittest.skip("can not test")
    def test_play_round():

        # Can not test due to hardcoded System.in use in Player.next_move
        pass
