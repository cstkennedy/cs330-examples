#! /usr/bin/env python

import copy

from house import House
from room import Room, RoomBuilder


def main():
    """
    Compute the area of a room and the cost of
    flooring for the room

    Let us Review the use of reference and pointer variables.

    We will use these when we implement the iterator interface.
    """

    # Construct, build, and print a house
    house = House()
    build_house(house)
    print(house)

    # Upgrade the flooring in a second duplicate house
    duplicate_house = upgrade_flooring(house)

    print(f"house == duplicate_house -> {house == duplicate_house}")
    print(f"&house == &duplicate_house -> {house is duplicate_house}")
    print()

    print(house)
    print(duplicate_house)

    # Get all the flooring costs with a 10% discount
    costs = [discount_flooring(room) for room in duplicate_house]
    #  costs = []
    #  for room in duplicate_house:
    #      costs.append(discount_flooring(room))

    for room_cost in costs:
        print(f"{room_cost:.2f}")

    # Print the sum, min, max -> D.R.Y!
    total = sum(costs)
    min_c = min(costs)
    max_c = max(costs)

    print(f"Total: {total:.2f}")
    print(f"Min  : {min_c:.2f}")
    print(f"Max  : {max_c:.2f}")


def build_house(house: House) -> None:
    """
    Build our example house
    """

    house.add_room(
        RoomBuilder()
        .with_name("Laundry Room")
        .with_dimensions(8, 4)
        .with_flooring("Laminate", 1.95)
        .build()
    )

    kitchen = (
        RoomBuilder()
        .with_name("Kitchen")
        .with_dimensions(20, 12)
        .with_flooring("Tile", 3.87)
        .build()
    )

    house.add_room(kitchen)

    house.add_room(
        RoomBuilder()
        .with_name("Storage Room")
        .with_dimensions(16, 16)
        .with_flooring("Birch Wood", 4.39)
        .build()
    )


def upgrade_flooring(original: House) -> House:
    """
    Take a room and change the flooring

    Args:
        original: House to change

    Returns:
        House with the updated flooring
    """

    modified = copy.deepcopy(original)

    for room in modified:
        room.set_flooring("Stone Bricks", 12.97)

    #  modified.set_name("After Stone Bricks")
    modified.name = "After Stone Bricks"  # Magic!!

    return modified


def discount_flooring(a_room: Room) -> float:
    """
    Take a room, discount the flooring cost by 90%.

    Returns:
        Discounted flooring cost
    """

    return 0.90 * a_room.flooring_cost()


if __name__ == "__main__":
    main()
