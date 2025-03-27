from renovation.house import House, HouseBuilder
from renovation.room import FlooringBuilder, Room, RoomBuilder

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

    # Construct the house within the build function
    house = build_house(ROOM_DATA)
    print(house)

    # Upgrade the flooring in a second duplicate house
    duplicate_house = upgrade_flooring(house)

    print(f"house == duplicate_house -> {house == duplicate_house}")
    print(f"&house == &duplicate_house -> {house is duplicate_house}")
    print()

    print(house)
    print()
    print(duplicate_house)
    print()

    # Get all the flooring costs with a 10% discount
    costs = [discount_flooring(room) for room in duplicate_house]

    for room_cost in costs:
        print(f"{room_cost:.2f}")

    print(f"Total: {sum(costs):.2f}")
    print(f"Min  : {min(costs):.2f}")
    print(f"Max  : {max(costs):.2f}")


def parse_room(line: str) -> Room:
    """
    Parse a single room line in the form...

    name length width unit_cost flooring_name

    Raises:
        ValueError if one or more tokens are invalid

        IndexError if at least one token (e.g., name or length) is missing
    """

    name, the_rest = line.strip().split(";")
    tokens = the_rest.split()

    # Grab length and width first
    length, width = [float(val) for val in tokens[0:2]]

    # The flooring name may have a space in it
    flr_type = " ".join(tokens[3:])
    flr_cost = float(tokens[2])

    # fmt: off
    return (
        RoomBuilder()
        .with_name(name)
        .with_dimensions(length, width)
        .with_flooring(
            FlooringBuilder()
            .with_name(flr_type)
            .with_cost(flr_cost)
            .build()
        )
        .build()
    )
    # fmt: on


def build_house(room_data: str) -> House:
    """
    Build our example house

    Raises:
        InvariantError if there is not at least one valid room line

        ValueError if one or more tokens are invalid (e.g, cannot be parsed)

        IndexError if at least one token (e.g., name or lenght) is missing
    """

    rooms = []
    for line in room_data.splitlines():
        # Skip blank lines
        if not line:
            continue

        rooms.append(parse_room(line))

    # fmt: off
    return (
        HouseBuilder()
        .with_rooms(rooms)
        .build()
    )
    # fmt: on


def upgrade_flooring(original: House) -> House:
    """
    Take an existing house and create a pseudo-duplicate by copying each room
    and changing each copy's flooring.

    Args:
        original: House to change

    Returns:
        A copy of the original house with the updated flooring
    """

    return (
        # fmt: off
        HouseBuilder()
        .with_name("After Stone Bricks")
        .with_rooms(
            (
                RoomBuilder.from_template(room)
                .substitute_flooring(
                    FlooringBuilder()
                    .with_name("Stone Bricks")
                    .with_cost(12.97)
                    .build()
                )
                .build()
                for room in original
            )
        )
        .build()
        # fmt: on
    )


def discount_flooring(a_room: Room) -> float:
    """
    Take a room, discount the flooring cost by 90%.

    Returns:
        Discounted flooring cost
    """

    return 0.90 * a_room.flooring_cost()


if __name__ == "__main__":
    main()
