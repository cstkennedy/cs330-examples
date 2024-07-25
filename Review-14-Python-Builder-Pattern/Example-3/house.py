from __future__ import annotations

from dataclasses import dataclass, field
from typing import Iterable, List, Optional, Self

from room import Room


DEFAULT_NAME = "House"


class House:
    def __init__(self, *, name: str, rooms: list[Room]):
        """
        This is the Default constructor. Start with the name set to "House" and
        an empty set of rooms (i.e., zero rooms).
        """

        self.__name: str = name
        self.__rooms: list[Room] = rooms

    @property
    def name(self):
        """
        Get the name using a property.

        @property must come before @...setter...
        """

        return self.__name

    def __len__(self):
        """
        Return the number of rooms in this House.
        """

        return len(self.__rooms)

    def flooring_cost_metrics(self) -> tuple[float, float]:
        #  total = sum(map(lambda room: room.flooring_cost(), self))
        total = sum(room.flooring_cost() for room in self)
        avg = total / len(self)

        return total, avg

    def __iter__(self):
        """
        Wrapper around `list.__iter__()`.
        """

        return iter(self.__rooms)

    def __str__(self) -> str:
        """
        This is the equivalent of overloading:
          - `operator<<` in C++
          - `toString` in Java
          - `Display::fmt` in Rust
        """

        total, avg = self.flooring_cost_metrics()

        # TODO: replace '+" with '"\n".join(...)'
        return (
            f"--------{self.name:}--------\n"
            + "\n".join(str(room) for room in self)
            + "------------------------------\n"
            + f"Total Cost   : $ {total:.2f}\n"
            + f"Avg Room Cost: $ {avg:.2f}\n"
        )

    def __eq__(self, rhs) -> bool:
        """
        This is the equivalent of overloading:
          - `operator==` in C++
          - `equals` in Java
          - `eq` in Rust
        """

        # TODO: Add instanceof check

        if self.name != rhs.name:
            return False

        if len(self) != len(rhs):
            return False

        # Python handles the traversal for us!
        return self.__rooms == rhs.__rooms


@dataclass
class HouseBuilder:
    name: Optional[str] = None
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
        if not self.the_rooms:
            raise ValueError("A House must have at least one room.")

        # If a name was supplied use it... otherwise supply the default.
        if self.name:
            return House(name=self.name, rooms=self.the_rooms)

        return House(name=DEFAULT_NAME, rooms=self.the_rooms)
