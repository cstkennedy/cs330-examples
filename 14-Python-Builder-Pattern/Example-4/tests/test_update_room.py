from hamcrest import assert_that, close_to, contains_exactly, equal_to, is_

import update_room
from renovation.house import HouseBuilder
from renovation.room import FlooringBuilder, RoomBuilder


def test_build_house():
    expected_rooms = [
        (
            RoomBuilder()
            .with_name("Laundry Room")
            .with_dimensions(8, 4)
            .with_flooring(
                FlooringBuilder().with_name("Laminate").with_cost(1.95).build()
            )
            .build()
        ),
        (
            RoomBuilder()
            .with_name("Kitchen")
            .with_dimensions(20, 12)
            .with_flooring(
                FlooringBuilder().with_name("Tile").with_cost(3.87).build()
            )
            .build()
        ),
        (
            RoomBuilder()
            .with_name("Storage Room")
            .with_dimensions(16, 16)
            .with_flooring(
                FlooringBuilder()
                .with_name("Birch Wood")
                .with_cost(4.39)
                .build()
            )
            .build()
        ),
    ]

    actual_house = update_room.build_house(update_room.ROOM_DATA)

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
                    .with_flooring(
                        FlooringBuilder()
                        .with_name("Laminate")
                        .with_cost(1.95)
                        .build()
                    )
                    .build()
                ),
                (
                    RoomBuilder()
                    .with_name("Kitchen")
                    .with_dimensions(20, 12)
                    .with_flooring(
                        FlooringBuilder()
                        .with_name("Tile")
                        .with_cost(3.87)
                        .build()
                    )
                    .build()
                ),
                (
                    RoomBuilder()
                    .with_name("Storage Room")
                    .with_dimensions(16, 16)
                    .with_flooring(
                        FlooringBuilder()
                        .with_name("Birch Wood")
                        .with_cost(4.39)
                        .build()
                    )
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
                    .with_flooring(
                        FlooringBuilder()
                        .with_name("Stone Bricks")
                        .with_cost(12.97)
                        .build()
                    )
                    .build()
                ),
                (
                    RoomBuilder()
                    .with_name("Kitchen")
                    .with_dimensions(20, 12)
                    .with_flooring(
                        FlooringBuilder()
                        .with_name("Stone Bricks")
                        .with_cost(12.97)
                        .build()
                    )
                    .build()
                ),
                (
                    RoomBuilder()
                    .with_name("Storage Room")
                    .with_dimensions(16, 16)
                    .with_flooring(
                        FlooringBuilder()
                        .with_name("Stone Bricks")
                        .with_cost(12.97)
                        .build()
                    )
                    .build()
                ),
            ]
        )
        .build()
    )

    house_upgraded = update_room.upgrade_flooring(house_original)

    assert_that(house_upgraded, is_(equal_to(house_expected)))


def test_discount_flooring():
    room = (
        RoomBuilder()
        .with_name("Generic Name")
        .with_dimensions(1, 2)
        .with_flooring(FlooringBuilder().with_name("Tile").with_cost(1).build())
        .build()
    )

    assert_that(update_room.discount_flooring(room), is_(close_to(1.8, 1e-2)))
