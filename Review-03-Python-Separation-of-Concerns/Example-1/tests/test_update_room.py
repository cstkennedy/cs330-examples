import pytest
from hamcrest import *

from room import Room
from update_room import discount_flooring


def test_build_house():
    pytest.fail("Test not yet written")


def test_upgrade_flooring():
    pytest.fail("Test not yet written")


def test_discount_flooring():
    room = (
        Room()
        .with_name("Generic Name")
        .with_dimensions(1, 2)
        .with_flooring("Tile", 1)
    )

    assert_that(discount_flooring(room), is_(close_to(1.8, 1e-2)))
