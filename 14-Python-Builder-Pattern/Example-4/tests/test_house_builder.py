import pytest
from hamcrest import assert_that, contains_exactly, equal_to, has_length, is_

from renovation.house import HouseBuilder
from renovation.room import FlooringBuilder, RoomBuilder


@pytest.fixture
def three_rooms():
    rooms = [
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

    builder = HouseBuilder().with_name("Test House!!!!")

    for room in rooms_to_add:
        _ = builder.with_room(room)

    house = builder.build()

    assert_that(house.name, is_(equal_to("Test House!!!!")))
    assert_that(house, has_length(num_rooms))
    assert_that(house, contains_exactly(*rooms_to_add))


@pytest.mark.parametrize("num_rooms", [1, 2, 3])
def test_with_name_and_n_rooms_at_once(three_rooms, num_rooms):
    rooms_to_add = three_rooms[0:num_rooms]

    builder = HouseBuilder().with_name("Test House!!!!")

    for room in rooms_to_add:
        _ = builder.with_room(room)

    house = builder.build()

    assert_that(house.name, is_(equal_to("Test House!!!!")))
    assert_that(house, has_length(num_rooms))
    assert_that(house, contains_exactly(*rooms_to_add))
