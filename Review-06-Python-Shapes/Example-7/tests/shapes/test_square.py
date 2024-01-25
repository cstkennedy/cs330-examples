import copy

import pytest
from hamcrest import *

from shapes.shape import FPT_FMT, Shape
from shapes.square import Square

"""
1 - Does this piece of code perform the operations
    it was designed to perform?

2 - Does this piece of code do something it was not
    designed to perform?

1 Test per mutator
"""

@pytest.fixture
def common_squares():
    generic = Square()
    fancy = Square(side=2)

    yield generic, fancy


def test_default_constructor(common_squares):
    generic, _ = common_squares
    assert_that(generic.name, equal_to("Square"))
    assert_that(generic.side, close_to(1, 1e-8))


def test_constructor(common_squares):
    _, fancy = common_squares
    assert_that(fancy.name, equal_to("Square"))
    assert_that(fancy.side, close_to(2.0, 1e-8))


def test_side_setter(common_squares):
    _, fancy = common_squares
    a_square = Square()

    a_square.side = 7.39

    assert_that(a_square.side, close_to(7.39, 1e-8))
    assert_that(a_square.name, equal_to("Square"))


def test_area(common_squares):
    generic, fancy = common_squares
    assert_that(generic.area(), close_to(generic.side**2, 1e-8))

    assert_that(fancy.area(), close_to(fancy.side**2, 1e-8))


def test_perimeter(common_squares):
    generic, fancy = common_squares
    assert_that(generic.perimeter(), close_to(4 * generic.side, 1e-8))

    assert_that(fancy.perimeter(), close_to(4 * fancy.side, 1e-8))


def test_deep_copy(common_squares):
    _, fancy = common_squares
    a_copy = copy.deepcopy(fancy)

    assert_that(a_copy, is_not(same_instance(fancy)))

    # I really should have defined __eq__
    assert_that(a_copy.side, close_to(fancy.side, 0.001))


def test_str(common_squares):
    _, fancy = common_squares
    fancy_str = str(fancy)

    assert_that(fancy_str, starts_with("Name"))
    assert_that(fancy_str, contains_string("Square"))
    assert_that(
        fancy_str,
        contains_string(FPT_FMT.format("Perimeter", fancy.perimeter())),
    )
    assert_that(fancy_str, contains_string(FPT_FMT.format("Area", fancy.area())))
    assert_that(fancy_str, contains_string(FPT_FMT.format("Side", fancy.side)))
    assert_that(fancy_str, ends_with("\n"))
