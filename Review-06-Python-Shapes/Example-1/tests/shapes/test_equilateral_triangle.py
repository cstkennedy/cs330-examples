import copy
import math

import pytest
from hamcrest import (assert_that, close_to, contains_string, ends_with,
                      equal_to, is_not, same_instance, starts_with)

from shapes.EquilateralTriangle import EquilateralTriangle
from shapes.Shape import FPT_FMT, Shape

"""
1 - Does this piece of code perform the operations
    it was designed to perform?

2 - Does this piece of code do something it was not
    designed to perform?

1 Test per mutator
"""


@pytest.fixture
def common_triangles():
    generic = EquilateralTriangle()
    fancy = EquilateralTriangle(side=3)

    yield generic, fancy


def test_default_constructor(common_triangles):
    generic, _ = common_triangles
    assert_that(generic.name, equal_to("Equilateral Triangle"))
    assert_that(generic.side, close_to(1, 1e-8))


def test_constructor(common_triangles):
    _, fancy = common_triangles
    assert_that(fancy.name, equal_to("Equilateral Triangle"))
    assert_that(fancy.side, close_to(3, 1e-8))


def test_side_setter():
    a_triangle = EquilateralTriangle()

    a_triangle.side = 7.39
    assert_that(a_triangle.side, close_to(7.39, 1e-8))


def test_area(common_triangles):
    generic, fancy = common_triangles
    # Based on 1/2 base * height (side=1)
    expected_area = math.sqrt(3) / 4
    assert_that(generic.area(), close_to(expected_area, 1e-8))

    # Based on 1/2 side * height (side=3)
    expected_area = 3 * math.sqrt(27) / 4
    assert_that(fancy.area(), close_to(expected_area, 1e-8))


def test_perimeter(common_triangles):
    generic, fancy = common_triangles
    assert_that(generic.perimeter(), close_to(3, 1e-8))
    assert_that(fancy.perimeter(), close_to(9, 1e-8))


def test_deep_copy(common_triangles):
    _, fancy = common_triangles
    a_copy = copy.deepcopy(fancy)

    assert_that(a_copy, is_not(same_instance(fancy)))

    # I really should have defined __eq__
    assert_that(a_copy.side, close_to(fancy.side, 1e-8))


def test_str(common_triangles):
    _, fancy = common_triangles
    fancy_str = str(fancy)

    assert_that(fancy_str, starts_with("Name"))
    assert_that(fancy_str, contains_string("Equilateral Triangle"))
    assert_that(
        fancy_str,
        contains_string(FPT_FMT.format("Perimeter", fancy.perimeter())),
    )
    assert_that(fancy_str, contains_string(FPT_FMT.format("Area", fancy.area())))

    assert_that(fancy_str, contains_string(FPT_FMT.format("Side", fancy.side)))

    assert_that(fancy_str, ends_with("\n"))
