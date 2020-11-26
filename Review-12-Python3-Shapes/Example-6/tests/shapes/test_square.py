from hamcrest import *
import unittest

from shapes.square import Square
from shapes.shape import (Shape, FPT_FMT)

import copy


class TestSquare(unittest.TestCase):
    """
    1 - Does this piece of code perform the operations
        it was designed to perform?

    2 - Does this piece of code do something it was not
        designed to perform?

    1 Test per mutator
    """

    def setUp(self):
        self.generic = Square()
        self.fancy = Square(side=2)

    def test_default_constructor(self):
        assert_that(self.generic.name, equal_to("Square"))
        assert_that(self.generic.side, close_to(1, 1e-8))

    def test_constructor(self):
        assert_that(self.fancy.name, equal_to("Square"))
        assert_that(self.fancy.side, close_to(2.0, 1e-8))

    def test_side_setter(self):
        a_square = Square()

        a_square.side = 7.39

        assert_that(a_square.side, close_to(7.39, 1e-8))
        assert_that(a_square.name, equal_to("Square"))

    def test_area(self):
        assert_that(self.generic.area(),
                    close_to(self.generic.side ** 2, 1e-8))

        assert_that(self.fancy.area(),
                    close_to(self.fancy.side ** 2, 1e-8))

    def test_perimeter(self):
        assert_that(self.generic.perimeter(),
                    close_to(4 * self.generic.side, 1e-8))

        assert_that(self.fancy.perimeter(),
                    close_to(4 * self.fancy.side, 1e-8))

    def test_deep_copy(self):
        a_copy = copy.deepcopy(self.fancy)

        assert_that(a_copy, is_not(same_instance(self.fancy)))

        # I really should have defined __eq__
        assert_that(a_copy.side, close_to(self.fancy.side, 0.001))

    def test_str(self):
        fancy_str = str(self.fancy)

        assert_that(fancy_str, starts_with("Name"))
        assert_that(fancy_str, contains_string("Square"))
        assert_that(fancy_str,
                    contains_string(FPT_FMT.format("Perimeter",
                                                   self.fancy.perimeter())))
        assert_that(fancy_str,
                    contains_string(FPT_FMT.format("Area",
                                                   self.fancy.area())))
        assert_that(fancy_str,
                    contains_string(FPT_FMT.format("Side",
                                                   self.fancy.side)))
        assert_that(fancy_str, ends_with("\n"))
