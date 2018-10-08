from hamcrest import *
import unittest

from examples.board import Board

import copy


class TestBoard(unittest.TestCase):
    """
    1 - Does this piece of code perform the operations
        it was designed to perform?

    2 - Does this piece of code do something it was not
        designed to perform?

    1 Test per mutator
    """
    expectedEmptyStr = "1|2|3\n4|5|6\n7|8|9"

    def setUp(self):

        self.aBoard = Board()

    def testDefaultConstructor(self):

        for expectedChar in range(1, 10):
            assert_that(self.aBoard.getCell(expectedChar),
                        is_(str(expectedChar)))

        assert_that(str(self.aBoard), equal_to(TestBoard.expectedEmptyStr))
        self.assertFalse(self.aBoard.isFull())

        retrieved = self.aBoard.get3Cells(1, 2, 3)
        expected = ((0, '1'), (1, '2'), (2, '3'))

        assert_that(retrieved[0], equal_to(expected[0]))
        assert_that(retrieved[1], equal_to(expected[1]))
        assert_that(retrieved[2], equal_to(expected[2]))

    def testSetCell(self):

        self.aBoard.setCell(1, 'X')
        self.aBoard.setCell(9, 'O')

        assert_that(self.aBoard.getCell(1), is_('X'))
        assert_that(self.aBoard.getCell(9), is_('O'))

        retrieved = self.aBoard.get3Cells(1, 5, 9)
        expected = ((0, 'X'), (4, '5'), (8, 'O'))

        assert_that(retrieved[0], equal_to(expected[0]))
        assert_that(retrieved[1], equal_to(expected[1]))
        assert_that(retrieved[2], equal_to(expected[2]))

        assert_that(str(self.aBoard),
                    is_not(equal_to(TestBoard.expectedEmptyStr)))
        assert_that(str(self.aBoard), equal_to("X|2|3\n4|5|6\n7|8|O"))

        self.assertFalse(self.aBoard.isFull())
