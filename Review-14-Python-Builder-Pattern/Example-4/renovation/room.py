from __future__ import annotations

import copy
from dataclasses import dataclass
from typing import Self

from .error import InvariantError, NotSetError


@dataclass
class Flooring:
    type_name: str
    unit_cost: float


@dataclass
class Room:
    name: str
    length: float
    width: float
    flooring: Flooring

    def area(self) -> float:
        """
        Compute the area of flooring for a room.
        """

        return self.length * self.width

    def flooring_cost(self) -> float:
        """
        Compute the flooring cost based on `self.area()` and unit cost.
        """

        return self.area() * self.flooring.unit_cost

    def __str__(self) -> str:
        return "\n".join(
            (
                f"Room ({self.name})",
                f"  Length: {self.length:>8.1f}",
                f"  Width : {self.width:>8.1f}",
                f"  Area  : {self.area():>8.1f}",
                "",
                f"  Flooring  : {self.flooring.type_name}",
                f"  Unit Cost : $ {self.flooring.unit_cost:>8.2f}",
                f"  Total Cost: $ {self.flooring_cost():>8.2f}",
            )
        )


class FlooringBuilder:
    def __init__(self) -> None:
        self.__name: str | None = None
        self.__unit_cost: float | None = None

    def with_name(self, name: str) -> Self:
        """
        Set the name after stripping any leading and trailing whitespace

        Raises:
            InvariantError if the trimmed name is not at least three (3)
            characters.
        """

        name = name.strip()

        if len(name) < 3:
            raise InvariantError("name must be at least 3 charcters")

        self.__name = name

        return self

    def with_cost(self, cost: float) -> Self:
        """
        Set the unit cost for one square unit (e.g., sq. ft.) of flooring.

        Args:
            l: length
            w: width

        Raises:
            InvariantError if cost is not at least 0.01
        """

        if cost <= 0:
            raise InvariantError("unit cost must be at least 0.01")

        self.__unit_cost = cost

        return self

    def build(self) -> Flooring:
        if not self.__name:
            raise NotSetError("'name' was not set")

        if not self.__unit_cost:
            raise NotSetError("'unit cost' was not set")

        return Flooring(self.__name, self.__unit_cost)


class RoomBuilder:
    def __init__(self) -> None:
        self.__name: str | None = None
        self.__length: float | None = None
        self.__width: float | None = None
        self.__flooring: Flooring | None = None

    @classmethod
    def from_template(cls, room: Room) -> RoomBuilder:
        bldr = cls()

        bldr.__name = copy.deepcopy(room.name)
        bldr.__length = copy.deepcopy(room.length)
        bldr.__width = copy.deepcopy(room.width)
        bldr.__flooring = copy.deepcopy(room.flooring)

        return bldr

    def substitute_flooring(self, flooring: Flooring) -> Self:
        self.with_flooring(flooring)

        return self

    def with_name(self, name: str) -> Self:
        """
        Set the name after stripping any leading and trailing whitespace

        Raises:
            InvariantError if the trimmed name is not at least three (3)
            characters.
        """

        name = name.strip()

        if len(name) < 3:
            raise InvariantError("name must be at least 3 charcters")

        self.__name = name

        return self

    def with_flooring(self, flooring: Flooring) -> Self:
        self.__flooring = flooring

        return self

    def with_dimensions(self, length: float, width: float) -> Self:
        """
        Set the Flooring using the builder pattern.

        Args:
            l: length
            w: width

        Raises:
            InvariantError if length or width is not greater than 0
        """

        if length <= 0:
            raise InvariantError("Length must be greater than 0")

        if width <= 0:
            raise InvariantError("Width must be greater than 0")

        self.__length = length
        self.__width = width

        return self

    def build(self) -> Room:
        if not self.__name:
            raise NotSetError("'name' was not set")

        if not self.__length:
            raise NotSetError("'length' was not set")

        if not self.__width:
            raise NotSetError("'width' was not set")

        if not self.__flooring:
            raise NotSetError("'flooring' was not set")

        room = Room(self.__name, self.__length, self.__width, self.__flooring)

        return room
