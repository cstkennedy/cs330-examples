from __future__ import annotations

from typing import Iterable, List, Optional, Self

from room import Room


class House:
    def __init__(self, nme: str = "House"):
        """
        This is the Default constructor. Start with the name set to "House" and
        an empty set of rooms (i.e., zero rooms).
        """

        self.__name: str = nme
        self.__rooms: list[Room] = []

    @property
    def name(self):
        """
        Get the name using a property.

        @property must come before @...setter...
        """

        return self.__name

    @name.setter
    def name(self, nme: str):
        """
        Set the name using a setter.

        Args:
            nme: new House name
        """

        self.__name = nme

    def add_room(self, to_add: Room):
        """
        Add another room to this House.

        Args:
            to_add: new Room to add
        """

        self.__rooms.append(to_add)

    def __len__(self):
        """
        Return the number of rooms in this House.
        """

        return len(self.__rooms)

    def is_empty(self) -> bool:
        """
        Determine whether this House is empty (i.e. `self.len() == 0).
        """

        return len(self) == 0

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


class HouseBuilder:
    def __init__(self) -> None:
        self.name: Optional[str] = None
        self.the_rooms: list[Room] = []

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
        pass
        """

        self.the_rooms.extend(new_rooms)

        return self

    def build(self) -> House:
        if not self.the_rooms:
            raise ValueError("A House must have at least one room.")

        # If a name was supplied use it... otherwise let the House constructor
        # supply the default.
        #  house = House(self.name) if self.name else House()
        house = House()
        if self.name:
            house.name = self.name

        house._House__rooms = self.the_rooms  # type: ignore

        return house
