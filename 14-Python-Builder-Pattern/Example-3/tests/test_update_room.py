from hamcrest import *

import update_room
from house import HouseBuilder
from room import RoomBuilder


def test_build_house():
    expected_rooms = [
        (
            RoomBuilder()
            .with_name("Laundry Room")
            .with_dimensions(8, 4)
            .with_flooring("Laminate", 1.95)
            .build()
        ),
        (
            RoomBuilder()
            .with_name("Kitchen")
            .with_dimensions(20, 12)
            .with_flooring("Tile", 3.87)
            .build()
        ),
        (
            RoomBuilder()
            .with_name("Storage Room")
            .with_dimensions(16, 16)
            .with_flooring("Birch Wood", 4.39)
            .build()
        ),
    ]

    actual_house = update_room.build_house()

    assert_that(actual_house.name, is_(equal_to("House")))
    assert_that(actual_house, contains_exactly(*expected_rooms))


def test_upgrade_flooring():
    house_original = (
        HouseBuilder()
        .with_name("House")
        .with_rooms(
            [
                (
                    RoomBuilder()
                    .with_name("Laundry Room")
                    .with_dimensions(8, 4)
                    .with_flooring("Laminate", 1.95)
                    .build()
                ),
                (
                    RoomBuilder()
                    .with_name("Kitchen")
                    .with_dimensions(20, 12)
                    .with_flooring("Tile", 3.87)
                    .build()
                ),
                (
                    RoomBuilder()
                    .with_name("Storage Room")
                    .with_dimensions(16, 16)
                    .with_flooring("Birch Wood", 4.39)
                    .build()
                ),
            ]
        )
        .build()
    )

    house_expected = (
        HouseBuilder()
        .with_name("After Stone Bricks")
        .with_rooms(
            [
                (
                    RoomBuilder()
                    .with_name("Laundry Room")
                    .with_dimensions(8, 4)
                    .with_flooring("Stone Bricks", 12.97)
                    .build()
                ),
                (
                    RoomBuilder()
                    .with_name("Kitchen")
                    .with_dimensions(20, 12)
                    .with_flooring("Stone Bricks", 12.97)
                    .build()
                ),
                (
                    RoomBuilder()
                    .with_name("Storage Room")
                    .with_dimensions(16, 16)
                    .with_flooring("Stone Bricks", 12.97)
                    .build()
                ),
            ]
        )
        .build()
    )

    house_upgraded = update_room.upgrade_flooring(house_original)

    assert_that(house_upgraded, is_(equal_to(house_expected)))


def test_discount_flooring():
    # fmt: off
    room = (
        RoomBuilder()
        .with_name("Generic Name")
        .with_dimensions(1, 2)
        .with_flooring("Tile", 1)
        .build()
    )
    # fmt: on

    assert_that(update_room.discount_flooring(room), is_(close_to(1.8, 1e-2)))
