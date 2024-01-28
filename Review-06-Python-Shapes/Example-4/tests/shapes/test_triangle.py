import copy
import math

import pytest
from hamcrest import *

from shapes.Shape import FPT_FMT, Shape
from shapes.Triangle import Triangle

"""
1 - Does this piece of code perform the operations
    it was designed to perform?

2 - Does this piece of code do something it was not
    designed to perform?

1 Test per mutator
"""


@pytest.fixture
def common_triangles():
    generic = Triangle()
    fancy = Triangle(a=1, b=1, c=math.sqrt(2))

    yield generic, fancy


def test_default_constructor(common_triangles):
    generic, _ = common_triangles
    assert_that(generic.name, equal_to("Triangle"))
    assert_that(generic.side_a, close_to(1, 1e-8))
    assert_that(generic.side_b, close_to(1, 1e-8))
    assert_that(generic.side_c, close_to(1, 1e-8))


def test_constructor(common_triangles):
    _, fancy = common_triangles
    assert_that(fancy.name, equal_to("Triangle"))
    assert_that(fancy.side_a, close_to(1, 1e-8))
    assert_that(fancy.side_b, close_to(1, 1e-8))
    assert_that(fancy.side_c, close_to(math.sqrt(2.0), 1e-8))


def test_side_a_setter():
    a_triangle = Triangle()

    a_triangle.side_a = 7.39

    assert_that(a_triangle.side_a, close_to(7.39, 1e-8))
    assert_that(a_triangle.side_b, close_to(1, 1e-8))
    assert_that(a_triangle.side_c, close_to(1, 1e-8))


def test_side_b_setter():
    a_triangle = Triangle()

    a_triangle.side_b = 7.39

    assert_that(a_triangle.side_a, close_to(1, 1e-8))
    assert_that(a_triangle.side_b, close_to(7.39, 1e-8))
    assert_that(a_triangle.side_c, close_to(1, 1e-8))


def test_side_c_setter():
    a_triangle = Triangle()

    a_triangle.side_c = 7.39

    assert_that(a_triangle.side_a, close_to(1, 1e-8))
    assert_that(a_triangle.side_b, close_to(1, 1e-8))
    assert_that(a_triangle.side_c, close_to(7.39, 1e-8))


def test_area(common_triangles):
    generic, fancy = common_triangles
    # Based on 1/2 base * height (base=1, height = sqrt(3)/2)
    expected_area = math.sqrt(3.0) / 4

    assert_that(generic.area(), close_to(expected_area, 1e-8))

    # Based on 1/2 base * height (base=1, height = 1)
    expected_area = 0.5

    assert_that(fancy.area(), close_to(expected_area, 1e-8))


def test_perimeter(common_triangles):
    generic, fancy = common_triangles
    assert_that(
        generic.perimeter(),
        close_to(
            sum([generic.side_a, generic.side_b, generic.side_c]),
            1e-8,
        ),
    )

    assert_that(
        fancy.perimeter(),
        close_to(sum([fancy.side_a, fancy.side_b, fancy.side_c]), 1e-8),
    )


def test_deep_copy(common_triangles):
    _, fancy = common_triangles
    a_copy = copy.deepcopy(fancy)

    assert_that(a_copy, is_not(same_instance(fancy)))

    # I really should have defined __eq__
    assert_that(a_copy.side_a, close_to(fancy.side_a, 1e-8))
    assert_that(a_copy.side_b, close_to(fancy.side_b, 1e-8))
    assert_that(a_copy.side_c, close_to(fancy.side_c, 1e-8))


def test_str(common_triangles):
    _, fancy = common_triangles
    fancy_str = str(fancy)

    assert_that(fancy_str, starts_with("Name"))
    assert_that(fancy_str, contains_string("Triangle"))

    assert_that(
        fancy_str,
        contains_string(FPT_FMT.format("Perimeter", fancy.perimeter())),
    )

    assert_that(fancy_str, contains_string(FPT_FMT.format("Area", fancy.area())))

    assert_that(fancy_str, contains_string(FPT_FMT.format("Side A", fancy.side_a)))

    assert_that(fancy_str, contains_string(FPT_FMT.format("Side B", fancy.side_b)))

    assert_that(fancy_str, contains_string(FPT_FMT.format("Side C", fancy.side_c)))

    assert_that(fancy_str, ends_with("\n"))
