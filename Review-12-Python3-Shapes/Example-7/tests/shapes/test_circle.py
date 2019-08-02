from hamcrest import *
import unittest

from shapes.circle import Circle
from shapes.shape import (Shape, FPT_FMT)

from shapes import circle
import copy


class TestCircle(unittest.TestCase):
    """
    1 - Does this piece of code perform the operations
        it was designed to perform?

    2 - Does this piece of code do something it was not
        designed to perform?

    1 Test per mutator
    """

    def setUp(self):
        self.generic = Circle()
        self.fancy = Circle(radius=2)

    def test_default_constructor(self):
        assert_that(self.generic.name, equal_to("Circle"))
        assert_that(self.generic.radius, close_to(1, 0.01))

    def test_constructor(self):
        assert_that(self.fancy.name, equal_to("Circle"))
        assert_that(self.fancy.radius, close_to(2.0, 1e-8))

    def test_radius_setter(self):
        a_circle = Circle()

        a_circle.radius = 7.39

        assert_that(a_circle.radius, close_to(7.39, 1e-8))

    def test_diameter(self):
        assert_that(self.generic.diameter, close_to(2, 1e-6))
        assert_that(self.fancy.diameter, close_to(4, 1e-6))

    def test_area(self):
        assert_that(self.generic.area(),
                    close_to(circle.PI * (self.generic.radius ** 2), 0.05))

        assert_that(self.fancy.area(),
                    close_to(circle.PI * (self.fancy.radius ** 2), 0.05))

    def test_perimeter(self):
        assert_that(self.generic.perimeter(),
                    close_to(circle.TAU * self.generic.radius, 0.05))

        assert_that(self.fancy.perimeter(),
                    close_to(circle.TAU * self.fancy.radius, 0.05))

    def test_deep_copy(self):
        a_copy = copy.deepcopy(self.fancy)

        assert_that(a_copy, is_not(same_instance(self.fancy)))

        # I really should have defined __eq__
        assert_that(a_copy.radius, close_to(self.fancy.radius, 0.001))

    def test_str(self):
        fancy_str = str(self.fancy)

        assert_that(fancy_str, starts_with("Name"))
        assert_that(fancy_str, contains_string("Circle"))
        assert_that(fancy_str, ends_with("\n"))

        assert_that(fancy_str,
                    contains_string(FPT_FMT.format("Perimeter",
                                                   self.fancy.perimeter())))
        assert_that(fancy_str,
                    contains_string(FPT_FMT.format("Area", self.fancy.area())))
        assert_that(fancy_str,
                    contains_string(FPT_FMT.format("Radius", self.fancy.radius)))
        assert_that(fancy_str,
                    contains_string(FPT_FMT.format("Diameter", self.fancy.diameter)))

        assert_that(fancy_str, ends_with("\n"))
