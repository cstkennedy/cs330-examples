from hamcrest import *
import unittest

from shapes.triangle import EquilateralTriangle
from shapes.shape import (Shape, FPT_FMT)

import math
import copy


class TestEquilateralTriangle(unittest.TestCase):
    """
    1 - Does this piece of code perform the operations
        it was designed to perform?

    2 - Does this piece of code do something it was not
        designed to perform?

    1 Test per mutator
    """

    def setUp(self):
        self.generic = EquilateralTriangle()
        self.fancy = EquilateralTriangle(side=3)

    def test_default_constructor(self):
        assert_that(self.generic.name, equal_to("Equilateral Triangle"))
        assert_that(self.generic.side, close_to(1, 1e-8))

    def test_constructor(self):
        assert_that(self.fancy.name, equal_to("Equilateral Triangle"))
        assert_that(self.fancy.side, close_to(3, 1e-8))

    def test_side_setter(self):
        a_triangle = EquilateralTriangle()

        a_triangle.side = 7.39
        assert_that(a_triangle.side, close_to(7.39, 1e-8))

    def test_area(self):
        # Based on 1/2 base * height (side=1)
        expected_area = math.sqrt(3) / 4
        assert_that(self.generic.area(),
                    close_to(expected_area, 1e-8))

        # Based on 1/2 side * height (side=3)
        expected_area = 3 * math.sqrt(27) / 4
        assert_that(self.fancy.area(), close_to(expected_area, 1e-8))

    def test_perimeter(self):
        assert_that(self.generic.perimeter(), close_to(3, 1e-8))
        assert_that(self.fancy.perimeter(), close_to(9, 1e-8))

    def test_deep_copy(self):
        a_copy = copy.deepcopy(self.fancy)

        assert_that(a_copy, is_not(same_instance(self.fancy)))

        # I really should have defined __eq__
        assert_that(a_copy.side, close_to(self.fancy.side, 1e-8))

    def test_str(self):
        fancy_str = str(self.fancy)

        assert_that(fancy_str, starts_with("Name"))
        assert_that(fancy_str, contains_string("Equilateral Triangle"))
        assert_that(fancy_str,
                    contains_string(FPT_FMT.format("Perimeter",
                                                   self.fancy.perimeter())))
        assert_that(fancy_str,
                    contains_string(FPT_FMT.format("Area", self.fancy.area())))

        assert_that(fancy_str,
                    contains_string(FPT_FMT.format("Side", self.fancy.side)))

        assert_that(fancy_str, ends_with("\n"))
