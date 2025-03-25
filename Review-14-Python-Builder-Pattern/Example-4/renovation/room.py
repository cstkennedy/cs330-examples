from __future__ import annotations

import copy
from dataclasses import dataclass, field
from typing import Optional, Self

from .error import NotSetValueError, BuildValueError


@dataclass
class DimensionSet:
    length: float
    width: float


@dataclass
class Flooring:
    type_name: str
    unit_cost: float


@dataclass
class Room:
    name: str
    dimensions: DimensionSet
    flooring: Flooring

    def area(self) -> float:
        """
        Compute the area of flooring for a room.
        """

        return self.dimensions.width * self.dimensions.length

    def flooring_cost(self) -> float:
        """
        Compute the flooring cost based on `self.area()` and unit cost.
        """

        return self.area() * self.flooring.unit_cost

    def __str__(self) -> str:
        return "\n".join(
            (
                f"Room ({self.name})",
                f"  Length: {self.dimensions.length:>8.1f}",
                f"  Width : {self.dimensions.width:>8.1f}",
                f"  Area  : {self.area():>8.1f}",
                "",
                f"  Flooring  : {self.flooring.type_name}",
                f"  Unit Cost : $ {self.flooring.unit_cost:>8.2f}",
                f"  Total Cost: $ {self.flooring_cost():>8.2f}",
                "\n",
            )
        )


class RoomBuilder:
    def __init__(self) -> None:
        self.__name: Optional[str] = None
        self.__length: Optional[float] = None
        self.__width: Optional[float] = None
        self.__flooring_type: Optional[str] = None
        self.__flooring_unit_cost: Optional[float] = None

    @classmethod
    def from_template(cls, room: Room) -> RoomBuilder:
        bldr = cls()

        bldr.__name = copy.deepcopy(room.name)
        bldr.__length = copy.deepcopy(room.dimensions.length)
        bldr.__width = copy.deepcopy(room.dimensions.width)
        bldr.__flooring_type = copy.deepcopy(room.flooring.type_name)
        bldr.__flooring_unit_cost = copy.deepcopy(room.flooring.unit_cost)

        return bldr

    def substitute_flooring(self, nme: str, unit_c: float) -> Self:
        self.with_flooring(nme, unit_c)

        return self

    def with_name(self, nme: str) -> Self:
        """
        Set the name using the builder pattern.

        Args:
            nme: room name
        """

        self.__name = nme.strip()

        return self

    def with_flooring(self, nme: str, unit_c: float) -> Self:
        """
        Set the Flooring using the builder pattern.

        Args
            nme: flooring type name
            unit_c: unit cost

        """

        self.__flooring_type = nme
        self.__flooring_unit_cost = unit_c

        return self

    def with_dimensions(self, l: float, w: float) -> Self:
        """
        Set the Flooring using the builder pattern.

        Args:
            l: length
            w: width

        """

        self.__length = l
        self.__width = w

        return self

    def __check_name(self, val: str | None, name: str) -> None:
        """
        Raises:
            ValueError if...
                1. val was not set
                2. val is the empty string
                3. val has fewer than 3 characters
        """

        if not val:
            raise NotSetValueError(f'"{name}" was not set')

        if len(val) < 3:
            raise ValueError(f'"{name}" len("{val}") < 3')

    def __check_num(self, val: float | int | None, name: str) -> None:
        """
        Raises:
            ValueError if...
                1. val was not set
                2. val is zero or negative
        """

        if not val:
            raise NotSetValueError(f'No "{name}" was set')

        if val <= 0:
            raise ValueError(f'"{name}" <= 0')

    def build(self) -> Room:
        try:
            self.__check_name(self.__name, "name")
            self.__check_name(self.__flooring_type, "flooring type")

            self.__check_num(self.__flooring_unit_cost, "flooring cost")
            self.__check_num(self.__length, "length")
            self.__check_num(self.__width, "width")

        except ValueError as err:
            raise BuildValueError from err

        room = Room(
            self.__name,  # type: ignore
            DimensionSet(self.__length, self.__width),
            Flooring(self.__flooring_type, self.__flooring_unit_cost), # type: ignore
        )

        return room
