from hamcrest import *
import unittest

from shapes.triangle import Triangle
from shapes.shape import (Shape, FPT_FMT)

import math
import copy


class TestTriangle(unittest.TestCase):
    """
    1 - Does this piece of code perform the operations
        it was designed to perform?

    2 - Does this piece of code do something it was not
        designed to perform?

    1 Test per mutator
    """

    def setUp(self):
        self.generic = Triangle()
        self.fancy = Triangle(a=1, b=1, c=math.sqrt(2))

    def test_default_constructor(self):
        assert_that(self.generic.name, equal_to("Triangle"))
        assert_that(self.generic.side_a, close_to(1, 1e-8))
        assert_that(self.generic.side_b, close_to(1, 1e-8))
        assert_that(self.generic.side_c, close_to(1, 1e-8))

    def test_constructor(self):
        assert_that(self.fancy.name, equal_to("Triangle"))
        assert_that(self.fancy.side_a, close_to(1, 1e-8))
        assert_that(self.fancy.side_b, close_to(1, 1e-8))
        assert_that(self.fancy.side_c, close_to(math.sqrt(2.0), 1e-8))

    def test_side_a_setter(self):
        a_triangle = Triangle()

        a_triangle.side_a = 7.39

        assert_that(a_triangle.side_a, close_to(7.39, 1e-8))
        assert_that(a_triangle.side_b, close_to(1, 1e-8))
        assert_that(a_triangle.side_c, close_to(1, 1e-8))

    def test_side_b_setter(self):
        a_triangle = Triangle()

        a_triangle.side_b = 7.39

        assert_that(a_triangle.side_a, close_to(1, 1e-8))
        assert_that(a_triangle.side_b, close_to(7.39, 1e-8))
        assert_that(a_triangle.side_c, close_to(1, 1e-8))

    def test_side_c_setter(self):
        a_triangle = Triangle()

        a_triangle.side_c = 7.39

        assert_that(a_triangle.side_a, close_to(1, 1e-8))
        assert_that(a_triangle.side_b, close_to(1, 1e-8))
        assert_that(a_triangle.side_c, close_to(7.39, 1e-8))

    def test_area(self):
        # Based on 1/2 base * height (base=1, height = sqrt(3)/2)
        expected_area = (math.sqrt(3.0) / 4)

        assert_that(self.generic.area(),
                    close_to(expected_area, 1e-8))

        # Based on 1/2 base * height (base=1, height = 1)
        expected_area = 0.5

        assert_that(self.fancy.area(),
                    close_to(expected_area, 1e-8))

    def test_perimeter(self):
        assert_that(self.generic.perimeter(),
                    close_to(sum([self.generic.side_a,
                                  self.generic.side_b,
                                  self.generic.side_c]), 1e-8))

        assert_that(self.fancy.perimeter(),
                    close_to(sum([self.fancy.side_a,
                                  self.fancy.side_b,
                                  self.fancy.side_c]), 1e-8))

    def test_deep_copy(self):
        a_copy = copy.deepcopy(self.fancy)

        assert_that(a_copy, is_not(same_instance(self.fancy)))

        # I really should have defined __eq__
        assert_that(a_copy.side_a, close_to(self.fancy.side_a, 1e-8))
        assert_that(a_copy.side_b, close_to(self.fancy.side_b, 1e-8))
        assert_that(a_copy.side_c, close_to(self.fancy.side_c, 1e-8))

    def test_str(self):
        fancy_str = str(self.fancy)

        assert_that(fancy_str, starts_with("Name"))
        assert_that(fancy_str, contains_string("Triangle"))

        assert_that(fancy_str,
                    contains_string(FPT_FMT.format("Perimeter", self.fancy.perimeter())))

        assert_that(fancy_str,
                    contains_string(FPT_FMT.format("Area", self.fancy.area())))

        assert_that(fancy_str,
                    contains_string(FPT_FMT.format("Side A", self.fancy.side_a)))

        assert_that(fancy_str,
                    contains_string(FPT_FMT.format("Side B", self.fancy.side_b)))

        assert_that(fancy_str,
                    contains_string(FPT_FMT.format("Side C", self.fancy.side_c)))

        assert_that(fancy_str, ends_with("\n"))
