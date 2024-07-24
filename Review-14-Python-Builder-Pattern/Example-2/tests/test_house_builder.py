import pytest
from hamcrest import *

from house import HouseBuilder
from room import RoomBuilder


@pytest.fixture
def three_rooms():
    rooms = [
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

    yield rooms


def test_zero_rooms():

    with pytest.raises(ValueError):
        _ = HouseBuilder().build()


@pytest.mark.parametrize("num_rooms", [1, 2, 3])
def test_with_room_n_times(three_rooms, num_rooms):

    rooms_to_add = three_rooms[0:num_rooms]

    builder = HouseBuilder()

    for room in rooms_to_add:
        _ = builder.with_room(room)

    house = builder.build()

    assert_that(house.name, is_(equal_to("House")))
    assert_that(house, has_length(num_rooms))
    assert_that(house, contains_exactly(*rooms_to_add))


@pytest.mark.parametrize("num_rooms", [1, 2, 3])
def test_with_rooms_n_at_once(three_rooms, num_rooms):

    rooms_to_add = three_rooms[0:num_rooms]

    house = HouseBuilder().with_rooms(rooms_to_add).build()

    assert_that(house.name, is_(equal_to("House")))
    assert_that(house, has_length(num_rooms))
    assert_that(house, contains_exactly(*rooms_to_add))


@pytest.mark.parametrize("num_rooms", [1, 2, 3])
def test_with_name_and_n_rooms(three_rooms, num_rooms):
    rooms_to_add = three_rooms[0:num_rooms]

    # ---------------------------------------------------------------------------
    # Using with_rooms
    # ---------------------------------------------------------------------------
    house = (
        HouseBuilder()
        .with_name("Test House!!!!")
        .with_rooms(rooms_to_add)
        .build()
    )

    assert_that(house.name, is_(equal_to("Test House!!!!")))
    assert_that(house, has_length(num_rooms))
    assert_that(house, contains_exactly(*rooms_to_add))

    # --------------------------------------------------------------------------
    # Using with_room
    # --------------------------------------------------------------------------
    builder = HouseBuilder().with_name("Test House!!!!")

    for room in rooms_to_add:
        _ = builder.with_room(room)

    house = builder.build()

    assert_that(house.name, is_(equal_to("Test House!!!!")))
    assert_that(house, has_length(num_rooms))
    assert_that(house, contains_exactly(*rooms_to_add))
