import copy

import pytest
from hamcrest import *

from shapes.Circle import Circle
from shapes.Shape import FPT_FMT, Shape

"""
1 - Does this piece of code perform the operations
    it was designed to perform?

2 - Does this piece of code do something it was not
    designed to perform?

1 Test per mutator
"""


@pytest.fixture
def common_circles():
    generic = Circle()
    fancy = Circle(radius=2)

    yield generic, fancy


def test_default_constructor(common_circles):
    generic, _ = common_circles
    assert_that(generic.name, equal_to("Circle"))
    assert_that(generic.radius, close_to(1, 0.01))


def test_constructor(common_circles):
    _, fancy = common_circles
    assert_that(fancy.name, equal_to("Circle"))
    assert_that(fancy.radius, close_to(2.0, 1e-8))


def test_radius_setter():
    a_circle = Circle()

    a_circle.radius = 7.39

    assert_that(a_circle.radius, close_to(7.39, 1e-8))


def test_diameter(common_circles):
    generic, fancy = common_circles
    assert_that(generic.diameter, close_to(2, 1e-6))
    assert_that(fancy.diameter, close_to(4, 1e-6))


def test_area(common_circles):
    generic, fancy = common_circles
    assert_that(generic.area(), close_to(Circle.PI * (generic.radius**2), 0.05))

    assert_that(fancy.area(), close_to(Circle.PI * (fancy.radius**2), 0.05))


def test_perimeter(common_circles):
    generic, fancy = common_circles
    assert_that(generic.perimeter(), close_to(Circle.TAU * generic.radius, 0.05))

    assert_that(fancy.perimeter(), close_to(Circle.TAU * fancy.radius, 0.05))


def test_deep_copy(common_circles):
    _, fancy = common_circles
    a_copy = copy.deepcopy(fancy)

    assert_that(a_copy, is_not(same_instance(fancy)))

    # I really should have defined __eq__
    assert_that(a_copy.radius, close_to(fancy.radius, 0.001))


def test_str(common_circles):
    _, fancy = common_circles
    fancy_str = str(fancy)

    assert_that(fancy_str, starts_with("Name"))
    assert_that(fancy_str, contains_string("Circle"))
    assert_that(fancy_str, ends_with("\n"))

    assert_that(
        fancy_str,
        contains_string(FPT_FMT.format("Perimeter", fancy.perimeter())),
    )
    assert_that(fancy_str, contains_string(FPT_FMT.format("Area", fancy.area())))
    assert_that(fancy_str, contains_string(FPT_FMT.format("Radius", fancy.radius)))
    assert_that(fancy_str, contains_string(FPT_FMT.format("Diameter", fancy.diameter)))

    assert_that(fancy_str, ends_with("\n"))
