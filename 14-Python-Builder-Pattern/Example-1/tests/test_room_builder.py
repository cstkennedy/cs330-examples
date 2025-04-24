import pytest
from hamcrest import *

from room import Room, RoomBuilder


def test_name_not_set():
    with pytest.raises(ValueError):
        # fmt: off
        _ = (
            RoomBuilder()
            .with_dimensions(1, 1)
            .with_flooring("Tile", 1.0)
            .build()
        )
        # fmt: on


@pytest.mark.parametrize("some_name", ["", "a", "ab"])
def test_name_too_short(some_name):
    with pytest.raises(ValueError):
        # fmt: off
        _ = (
            RoomBuilder()
            .with_name(some_name)
            .with_dimensions(1, 1)
            .with_flooring("Tile", 1.0)
            .build()
        )
        # fmt: on


def test_flooring_not_set():
    with pytest.raises(ValueError):
        # fmt: off
        _ = (
            RoomBuilder()
            .with_name("Generic Room")
            .with_dimensions(1, 1)
            .build()
        )
        # fmt: on


def test_length_not_set():
    with pytest.raises(ValueError):
        # fmt: off
        _ = (
            RoomBuilder()
            .with_name("Generic Name")
            .with_flooring("Tile", 1.0)
            .build()
        )
        # fmt: on


def test_width_not_set():
    pytest.skip("This should never happen if length is set")


def test_build_success():
    room = (
        RoomBuilder()
        .with_name("Generic Name")
        .with_dimensions(1, 2)
        .with_flooring("Tile", 1)
        .build()
    )

    assert_that(room.name, is_(equal_to("Generic Name")))
    assert_that(room._Room__dimensions.length, is_(close_to(1, 1e-8)))
    assert_that(room._Room__dimensions.width, is_(close_to(2, 1e-8)))
    assert_that(room._Room__flooring.type_name, is_(equal_to("Tile")))
    assert_that(room._Room__flooring.unit_cost, is_(close_to(1, 1e-2)))
