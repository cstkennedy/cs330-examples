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
    expected_empty_str = "1|2|3\n4|5|6\n7|8|9"

    def setUp(self):

        self.a_board = Board()

    def test_default_constructor(self):

        for expected_char in range(1, 10):
            assert_that(self.a_board.get_cell(expected_char),
                        is_(str(expected_char)))

        assert_that(str(self.a_board), equal_to(TestBoard.expected_empty_str))
        self.assertFalse(self.a_board.is_full())

        retrieved = self.a_board.get_3_cells(1, 2, 3)
        expected = ((0, '1'), (1, '2'), (2, '3'))

        assert_that(retrieved[0], equal_to(expected[0]))
        assert_that(retrieved[1], equal_to(expected[1]))
        assert_that(retrieved[2], equal_to(expected[2]))

    def test_set_cell(self):

        self.a_board.set_cell(1, 'X')
        self.a_board.set_cell(9, 'O')

        assert_that(self.a_board.get_cell(1), is_('X'))
        assert_that(self.a_board.get_cell(9), is_('O'))

        retrieved = self.a_board.get_3_cells(1, 5, 9)
        expected = ((0, 'X'), (4, '5'), (8, 'O'))

        assert_that(retrieved[0], equal_to(expected[0]))
        assert_that(retrieved[1], equal_to(expected[1]))
        assert_that(retrieved[2], equal_to(expected[2]))

        assert_that(str(self.a_board),
                    is_not(equal_to(TestBoard.expected_empty_str)))
        assert_that(str(self.a_board), equal_to("X|2|3\n4|5|6\n7|8|O"))

        self.assertFalse(self.a_board.is_full())
