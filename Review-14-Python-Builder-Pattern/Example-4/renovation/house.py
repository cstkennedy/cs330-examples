from __future__ import annotations

from dataclasses import dataclass, field
from itertools import chain
from typing import Iterable, Iterator, Self

from .error import InvariantError
from .room import Room

DEFAULT_NAME = "House"


class House:
    def __init__(self, *, name: str, rooms: list[Room]) -> None:
        """
        Start with the name set to "House" and an empty collection of rooms
        (i.e., zero rooms).
        """

        self.__name: str = name
        self.__rooms: list[Room] = rooms

    @property
    def name(self) -> str:
        return self.__name

    def __len__(self) -> int:
        """
        Return the number of rooms in this House.
        """

        return len(self.__rooms)

    def flooring_cost_metrics(self) -> tuple[float, float]:
        total = sum(room.flooring_cost() for room in self)
        avg = total / len(self)

        return total, avg

    def __iter__(self) -> Iterator[Room]:
        return iter(self.__rooms)

    def __str__(self) -> str:
        """
        This is the equivalent of...
          - overloading ``operator<<`` in C++
          - overriding ``toString`` in Java
          - implementing the ``Display`` trait in Rust
        """

        total, avg = self.flooring_cost_metrics()

        return "\n".join(
            chain(
                (f"--------{self.name:}--------\n",),
                (f"{room}\n" for room in self),
                (
                    "------------------------------",
                    f"Total Cost   : $ {total:.2f}",
                    f"Avg Room Cost: $ {avg:.2f}",
                ),
            )
        )

    def __eq__(self, rhs) -> bool:
        """
        This is the equivalent of:
          - overloading ``operator==`` in C++
          - overriding ``equals`` in Java
          - implementing or deriving the ``PartialEq`` trait in Rust
        """

        if not isinstance(rhs, House):
            return False

        if self.name != rhs.name:
            return False

        if len(self) != len(rhs):
            return False

        # Python handles the traversal for us!
        return self.__rooms == rhs.__rooms


@dataclass
class HouseBuilder:
    """
    Used to construct a house object. Setting a name is optional. At least one
    room must be added before invoking ``HouseBuilder.build``
    """

    name: str = DEFAULT_NAME
    the_rooms: list[Room] = field(default_factory=list)

    def with_name(self, nme: str) -> Self:
        """
        Set house name. This is optional.
        """
        self.name = nme

        return self

    def with_room(self, room: Room) -> Self:
        """
        Add a single room
        """
        self.the_rooms.append(room)

        return self

    def with_rooms(self, new_rooms: Iterable[Room]) -> Self:
        """
        Add multiple rooms from any iterable object
        """

        self.the_rooms.extend(new_rooms)

        return self

    def build(self) -> House:
        """
        Construct and return a house.


        Raises:
            InvariantError if there has not been at least one room provided
            through either ``with_room`` or ``with_rooms``
        """

        if not self.the_rooms:
            raise InvariantError("A House must have at least one room.")

        return House(name=self.name, rooms=self.the_rooms)
