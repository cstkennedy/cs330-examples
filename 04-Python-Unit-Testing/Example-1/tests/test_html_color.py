"""
1 - Does this piece of code perform the operations
    it was designed to perform?

2 - Does this piece of code do something it was not
    designed to perform?

1 Test per mutator
"""

import copy

import pytest
from hamcrest import *

from html_color import HtmlColor


@pytest.fixture
def black() -> HtmlColor:
    yield HtmlColor()


@pytest.fixture
def white() -> HtmlColor:
    yield HtmlColor(255, 255, 255)


@pytest.fixture
def blue() -> HtmlColor:
    yield HtmlColor(0, 0, 255)


def test_default_constructor(black: HtmlColor):
    color = HtmlColor()

    assert_that(color.red, is_(0))
    assert_that(color.green, is_(0))
    assert_that(color.blue, is_(0))

    assert_that(hash(color), is_(hash(black)))
    assert_that(str(color), equal_to("#000000"))


def test_non_default_constructor(black: HtmlColor):
    color = HtmlColor(7, 62, 55)

    assert_that(color.red, is_(7))
    assert_that(color.green, is_(62))
    assert_that(color.blue, is_(55))

    assert_that(hash(color), is_not(hash(black)))
    assert_that(str(color), equal_to("#073E37"))


def test_constructor_invalid_red():
    with pytest.raises(HtmlColor.InvalidColorException):
        _ = HtmlColor(-1, 62, 55)

    with pytest.raises(HtmlColor.InvalidColorException):
        _ = HtmlColor(700, 62, 55)


def test_constructor_invalid_green():
    with pytest.raises(HtmlColor.InvalidColorException):
        _ = HtmlColor(1, -1, 55)

    with pytest.raises(HtmlColor.InvalidColorException):
        _ = HtmlColor(1, 700, 55)


def test_constructor_invalid_blue():
    with pytest.raises(HtmlColor.InvalidColorException):
        _ = HtmlColor(0, 0, -10)

    with pytest.raises(HtmlColor.InvalidColorException):
        _ = HtmlColor(0, 0, 300)


def test_set_red(black: HtmlColor):
    color = copy.deepcopy(black)
    old_hash_code = hash(color)

    color.red = 100

    assert_that(color.red, is_(100))
    assert_that(color.blue, is_(0))
    assert_that(color.green, is_(0))

    assert_that(color, is_(equal_to(color)))
    assert_that(color, is_not(equal_to(black)))
    assert_that(hash(color), is_not(old_hash_code))


def test_set_green(blue: HtmlColor):
    color = copy.deepcopy(blue)
    old_hash_code = hash(color)

    color.green = 100

    assert_that(color.red, is_(0))
    assert_that(color.green, is_(100))
    assert_that(color.blue, is_(255))

    assert_that(color, is_(equal_to(color)))
    assert_that(color, is_not(equal_to(blue)))
    assert_that(hash(color), is_not(old_hash_code))


def test_set_blue(white: HtmlColor):
    color = copy.deepcopy(white)
    old_hash_code = hash(color)

    color.blue = 100

    assert_that(color.red, is_(255))
    assert_that(color.green, is_(255))
    assert_that(color.blue, is_(100))

    assert_that(color, is_(equal_to(color)))
    assert_that(color, is_not(equal_to(white)))
    assert_that(hash(color), is_not(old_hash_code))

    assert_that(color, has_string("#FFFF64"))
