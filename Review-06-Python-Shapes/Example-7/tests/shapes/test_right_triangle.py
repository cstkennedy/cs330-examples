from hamcrest import *
import unittest

from shapes.triangle import RightTriangle
from shapes.shape import (Shape, FPT_FMT)

import math
import copy


class TestRightTriangle(unittest.TestCase):
    """
    1 - Does this piece of code perform the operations
        it was designed to perform?

    2 - Does this piece of code do something it was not
        designed to perform?

    1 Test per mutator
    """

    def setUp(self):
        self.generic = RightTriangle()
        self.fancy = RightTriangle(base=2, height=3)

    def test_default_constructor(self):
        assert_that(self.generic.name, equal_to("Right Triangle"))
        assert_that(self.generic.base, close_to(1, 1e-8))
        assert_that(self.generic.height, close_to(1, 1e-8))
        assert_that(self.generic.hypotenuse, close_to(math.sqrt(2), 1e-8))

    def test_constructor(self):
        assert_that(self.fancy.name, equal_to("Right Triangle"))
        assert_that(self.fancy.base, close_to(2, 1e-8))
        assert_that(self.fancy.height, close_to(3, 1e-8))
        assert_that(self.fancy.hypotenuse, close_to(math.sqrt(13), 1e-8))

    def test_base_setter(self):
        a_triangle = RightTriangle()

        a_triangle.base = 7.39

        assert_that(a_triangle.base, close_to(7.39, 1e-8))
        assert_that(a_triangle.height, close_to(1, 1e-8))
        assert_that(a_triangle.hypotenuse, close_to(7.4573, 1e-4))

    def test_height_setter(self):
        a_triangle = RightTriangle()

        a_triangle.height = 7.39

        assert_that(a_triangle.base, close_to(1, 1e-8))
        assert_that(a_triangle.height, close_to(7.39, 1e-8))
        assert_that(a_triangle.hypotenuse, close_to(7.4573, 1e-4))

    def test_area(self):
        # Based on 1/2 base * height (base=1, height = 1)
        expected_area = 0.5

        assert_that(self.generic.area(),
                    close_to(expected_area, 1e-8))

        # Based on 1/2 base * height (base=2, height = 3)
        expected_area = 3

        assert_that(self.fancy.area(),
                    close_to(expected_area, 1e-8))

    def test_perimeter(self):
        assert_that(self.generic.perimeter(),
                    close_to(sum([self.generic.base,
                                  self.generic.height,
                                  self.generic.hypotenuse]), 1e-8))

        assert_that(self.fancy.perimeter(),
                    close_to(sum([self.fancy.base,
                                  self.fancy.height,
                                  self.fancy.hypotenuse]), 1e-8))

    def test_deep_copy(self):
        a_copy = copy.deepcopy(self.fancy)

        assert_that(a_copy, is_not(same_instance(self.fancy)))

        # I really should have defined __eq__
        assert_that(a_copy.base, close_to(self.fancy.base, 1e-8))
        assert_that(a_copy.height, close_to(self.fancy.height, 1e-8))
        assert_that(a_copy.hypotenuse, close_to(self.fancy.hypotenuse, 1e-8))

    def test_str(self):
        fancy_str = str(self.fancy)

        assert_that(fancy_str, starts_with("Name"))
        assert_that(fancy_str, contains_string("Right Triangle"))
        assert_that(fancy_str,
                    contains_string(FPT_FMT.format("Perimeter",
                                                   self.fancy.perimeter())))
        assert_that(fancy_str,
                    contains_string(FPT_FMT.format("Area",
                                                   self.fancy.area())))
        assert_that(fancy_str,
                    contains_string(FPT_FMT.format("Base",
                                                   self.fancy.base)))
        assert_that(fancy_str,
                    contains_string(FPT_FMT.format("Height",
                                                   self.fancy.height)))
        assert_that(fancy_str,
                    contains_string(FPT_FMT.format("Hypotenuse",
                                                   self.fancy.hypotenuse)))
        assert_that(fancy_str, ends_with("\n"))
