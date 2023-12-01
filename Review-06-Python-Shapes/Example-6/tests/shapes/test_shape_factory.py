import copy

import pytest
from hamcrest import *

import shapes.shape_factory as shape_factory
from shapes.shape import Shape

"""
**This is technically a set of Integration Tests**

1 - Does this piece of code perform the operations
    it was designed to perform?

2 - Does this piece of code do something it was not
    designed to perform?

1 Test per mutator
"""


@pytest.fixture
def known_shape_names_and_number():
    known_names = (
        "Circle",
        "Square",
        "Triangle",
        "Equilateral Triangle",
        "Right Triangle",
    )

    number_known = len(known_names)

    yield known_names, number_known


def test_create_shape_success():
    """
    Create a known valid Shape
    """

    assert_that(shape_factory.create("Circle"), is_(not_none()))


def test_create_shape_failure():
    """
    Try to create a known invalid Shape
    """

    assert_that(shape_factory.create("Lol Nope"), is_(None))


def test_is_known_success():
    """
    Create a known valid Shape
    """

    assert_that(shape_factory.is_known("Circle"))


def test_is_known_failure():
    """
    Try to create a known invalid Shape
    """

    assert_that(shape_factory.is_known("Lol Nope"), is_(False))


def test_list_known(known_shape_names_and_number):
    known_names, _ = known_shape_names_and_number

    known_str = shape_factory.list_known()

    for name in known_names:
        assert_that(known_str, contains_string(name))


def test_number_known(known_shape_names_and_number):
    _, number_known = known_shape_names_and_number
    assert_that(shape_factory.number_known(), equal_to(number_known))
