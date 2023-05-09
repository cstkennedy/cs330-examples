#! /usr/bin/env python

from room import Room


def main():
    """
    Compute the area of a room and the cost of flooring for the room.
    """

    room = Room("Laundry Room", 8, 4, 1.95)
    kitchen = Room("Kitchen", 20, 12, 3.87)

    room_pointer = room

    room.set_flooring("Tile", 2.50)

    print("----------------------------------------------------------------")
    print(room)
    print("----------------------------------------------------------------")
    print(room_pointer)
    print("----------------------------------------------------------------")

    print(
        f"&room ({id(room):#0x}) == &room_pointer ({id(room_pointer):#0x})"
        f" -> {room == room_pointer}"
    )
    print("----------------------------------------------------------------")

    if room == room_pointer:
        print("room == room_pointer")
    else:
        print("room != room_pointer")

    print("----------------------------------------------------------------")

    if room == kitchen:
        print("room == kitchen")
    else:
        print("room != kitchen")

    if room < kitchen:
        print("room < kitchen")
    else:
        print("room >= kitchen")

    print()


if __name__ == "__main__":
    main()
