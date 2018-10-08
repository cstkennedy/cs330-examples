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

        self.tom       = Player("Tom")
        self.aCylon    = Player()

        self.emptyBoard = Board()

        self.aGame = Game(self.tom, self.aCylon)

    def testConstructor(self):

        assert_that(self.aGame.getPlayer1(), equal_to(self.tom))
        assert_that(self.aGame.getPlayer2(), equal_to(self.aCylon))

        assert_that(self.aGame.getPlayer1().getSymbol(), is_('X'))
        assert_that(self.aGame.getPlayer2().getSymbol(), is_('O'))

        assert_that(self.aGame.isOver(), is_(False))

        assert_that(self.aGame.getWinner(), is_(none()))
        assert_that(self.aGame.getLoser(), is_(none()))

        # Can not test without Board.equals method
        assert_that(self.aGame.getBoard(), equal_to(self.emptyBoard))

    @unittest.skip("can not test")
    def testPlayRound():

        # Can not test due to hardcoded System.in use in Player.nextMove
        pass
