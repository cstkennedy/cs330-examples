from hamcrest import *
import unittest

from examples import (Player, Referee, Board, Game)

import copy


class TestReferee(unittest.TestCase):
    """
    1 - Does this piece of code perform the operations
        it was designed to perform?

    2 - Does this piece of code do something it was not
        designed to perform?

    1 Test per mutator
    """

    def setUp(self):

        self.emptyBoard = Board()

        self.aReferee = Referee(self.emptyBoard)

    def testConstructor(self):

        assert_that(self.aReferee.checkForWin(), is_(0))

        for i in range(1, 10):
            assert_that(self.aReferee.selectedCellIsEmpty(i), is_(True))

    def testCheckForHorizontalWin(self):

        hBoard = Board()

        hBoard.setCell(4, 'X')
        hBoard.setCell(5, 'X')
        hBoard.setCell(6, 'X')

        hReferee = Referee(hBoard)

        assert_that(hReferee.selectedCellIsEmpty(4), is_(False))
        assert_that(hReferee.selectedCellIsEmpty(5), is_(False))
        assert_that(hReferee.selectedCellIsEmpty(6), is_(False))

        assert_that(hReferee.checkForWin(), is_(1))

    def testCheckForVerticalWin(self):

        vBoard = Board()

        vBoard.setCell(2, 'O')
        vBoard.setCell(5, 'O')
        vBoard.setCell(8, 'O')

        vReferee = Referee(vBoard)

        assert_that(vReferee.selectedCellIsEmpty(2), is_(False))
        assert_that(vReferee.selectedCellIsEmpty(5), is_(False))
        assert_that(vReferee.selectedCellIsEmpty(8), is_(False))

        assert_that(vReferee.checkForWin(), is_(2))

    def testCheckForDiagonalWin(self):

        dBoard = Board()

        dBoard.setCell(3, 'O')
        dBoard.setCell(5, 'O')
        dBoard.setCell(7, 'O')

        dReferee = Referee(dBoard)

        assert_that(dReferee.selectedCellIsEmpty(3), is_(False))
        assert_that(dReferee.selectedCellIsEmpty(5), is_(False))
        assert_that(dReferee.selectedCellIsEmpty(7), is_(False))

        assert_that(dReferee.checkForWin(), is_(2))
