#! /usr/bin/env python

import copy

from house import House
from room import Room, RoomBuilder

ROOM_DATA = """
Laundry Room; 8 4 1.95 Laminate
Kitchen; 20 12 3.87 Tile
Storage Room; 16 16 4.39 Birch Wood
"""


def main():
    """
    Compute the area of a room and the cost of
    flooring for the room

    Let us Review the use of reference and pointer variables.

    We will use these when we implement the iterator interface.
    """

    house = House()

    # std::istringstream fakeInputFile(ROOM_DATA)
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

    for room_cost in costs:
        print(f"{room_cost:.2f}")

    # Print the sum, min, max -> D.R.Y!
    total = sum(costs)
    min_c = min(costs)
    max_c = max(costs)

    print(f"Total: {total:.2f}")
    print(f"Min  : {min_c:.2f}")
    print(f"Max  : {max_c:.2f}")


def build_house(house: House):
    """
    Build our example house
    """

    # Laundry Room; 8 4 1.95 Laminate
    for line in ROOM_DATA.splitlines():
        # Skip blank lines
        if not line:
            continue

        name, the_rest = line.strip().split(";")
        the_rest = the_rest.split()
        #  print(the_rest)

        # Grab length and width first
        length, width = [float(val) for val in the_rest[0:2]]

        # The flooring name may have a space in it
        flr_type = " ".join(the_rest[3:])
        flr_cost = float(the_rest[2])

        house.add_room(
            RoomBuilder()
            .with_name(name)
            .with_dimensions(length, width)
            .with_flooring(flr_type, flr_cost)
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
