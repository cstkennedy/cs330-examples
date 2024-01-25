import copy
import math

import pytest
from hamcrest import *

from shapes.shape import FPT_FMT, Shape
from shapes.triangle import RightTriangle

"""
1 - Does this piece of code perform the operations
    it was designed to perform?

2 - Does this piece of code do something it was not
    designed to perform?

1 Test per mutator
"""


@pytest.fixture
def common_triangles():
    generic = RightTriangle()
    fancy = RightTriangle(base=2, height=3)

    yield generic, fancy


def test_default_constructor(common_triangles):
    generic, fancy = common_triangles
    assert_that(generic.name, equal_to("Right Triangle"))
    assert_that(generic.base, close_to(1, 1e-8))
    assert_that(generic.height, close_to(1, 1e-8))
    assert_that(generic.hypotenuse, close_to(math.sqrt(2), 1e-8))


def test_constructor(common_triangles):
    generic, fancy = common_triangles
    assert_that(fancy.name, equal_to("Right Triangle"))
    assert_that(fancy.base, close_to(2, 1e-8))
    assert_that(fancy.height, close_to(3, 1e-8))
    assert_that(fancy.hypotenuse, close_to(math.sqrt(13), 1e-8))


def test_base_setter(common_triangles):
    generic, fancy = common_triangles
    a_triangle = RightTriangle()

    a_triangle.base = 7.39

    assert_that(a_triangle.base, close_to(7.39, 1e-8))
    assert_that(a_triangle.height, close_to(1, 1e-8))
    assert_that(a_triangle.hypotenuse, close_to(7.4573, 1e-4))


def test_height_setter(common_triangles):
    generic, fancy = common_triangles
    a_triangle = RightTriangle()

    a_triangle.height = 7.39

    assert_that(a_triangle.base, close_to(1, 1e-8))
    assert_that(a_triangle.height, close_to(7.39, 1e-8))
    assert_that(a_triangle.hypotenuse, close_to(7.4573, 1e-4))


def test_area(common_triangles):
    generic, fancy = common_triangles
    # Based on 1/2 base * height (base=1, height = 1)
    expected_area = 0.5

    assert_that(generic.area(), close_to(expected_area, 1e-8))

    # Based on 1/2 base * height (base=2, height = 3)
    expected_area = 3

    assert_that(fancy.area(), close_to(expected_area, 1e-8))


def test_perimeter(common_triangles):
    generic, fancy = common_triangles
    assert_that(
        generic.perimeter(),
        close_to(
            sum([generic.base, generic.height, generic.hypotenuse]),
            1e-8,
        ),
    )

    assert_that(
        fancy.perimeter(),
        close_to(sum([fancy.base, fancy.height, fancy.hypotenuse]), 1e-8),
    )


def test_deep_copy(common_triangles):
    generic, fancy = common_triangles
    a_copy = copy.deepcopy(fancy)

    assert_that(a_copy, is_not(same_instance(fancy)))

    # I really should have defined __eq__
    assert_that(a_copy.base, close_to(fancy.base, 1e-8))
    assert_that(a_copy.height, close_to(fancy.height, 1e-8))
    assert_that(a_copy.hypotenuse, close_to(fancy.hypotenuse, 1e-8))


def test_str(common_triangles):
    generic, fancy = common_triangles
    fancy_str = str(fancy)

    assert_that(fancy_str, starts_with("Name"))
    assert_that(fancy_str, contains_string("Right Triangle"))
    assert_that(
        fancy_str,
        contains_string(FPT_FMT.format("Perimeter", fancy.perimeter())),
    )
    assert_that(fancy_str, contains_string(FPT_FMT.format("Area", fancy.area())))
    assert_that(fancy_str, contains_string(FPT_FMT.format("Base", fancy.base)))
    assert_that(fancy_str, contains_string(FPT_FMT.format("Height", fancy.height)))
    assert_that(
        fancy_str,
        contains_string(FPT_FMT.format("Hypotenuse", fancy.hypotenuse)),
    )
    assert_that(fancy_str, ends_with("\n"))
