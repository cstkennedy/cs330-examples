from __future__ import annotations

import copy
from dataclasses import dataclass, field
from typing import Never, Optional, Self


@dataclass
class DimensionSet:
    length: float
    width: float


@dataclass
class Flooring:
    type_name: str
    unit_cost: float

    def update(self, nme: str, unit_c: float) -> None:
        """
        Replace/update the flooring.

        Args:
            nme: flooring type name
            unit_c: unit cost

        """

        self.type_name = nme
        self.unit_cost = unit_c


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

    def __deepcopy__(self, memo) -> Room:
        cpy = Room(
            self.name, copy.deepcopy(self.dimensions), copy.deepcopy(self.flooring)
        )

        return cpy

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

    def from_template(self, room: Room) -> Self:
        self.__name = copy.deepcopy(room.name)
        self.__length = copy.deepcopy(room.dimensions.length)
        self.__width = copy.deepcopy(room.dimensions.width)
        self.__flooring_type = copy.deepcopy(room.flooring.type_name)
        self.__flooring_unit_cost = copy.deepcopy(room.flooring.unit_cost)

        return self

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
        Raise a Value Error if:
          1. val was not set
          2. val is the empty string
          3. val has fewer than 3 characters
        """

        if not val:
            raise ValueError(f'"{name}" was not set')

        if len(val) < 3:
            raise ValueError('"{name}" len("{val}") < 3')

    def __check_num(self, val: float | int, name: str) -> None:
        if val <= 0:
            raise ValueError(f'"{name}" <= 0')

    def build(self) -> Room:
        self.__check_name(self.__name, "name")
        self.__check_name(self.__flooring_type, "flooring type")

        # TODO: Duplicate check (remove)
        if not self.__flooring_type:
            raise ValueError("No flooring type was set")

        if not self.__flooring_unit_cost:
            raise ValueError("No flooring cost was set")

        if not self.__length:
            raise ValueError("No length was set")

        if not self.__width:
            raise ValueError("No width was set")

        self.__check_num(self.__flooring_unit_cost, "flooring cost")
        self.__check_num(self.__length, "length")
        self.__check_num(self.__width, "width")

        room = Room(
            self.__name,  # type: ignore
            DimensionSet(self.__length, self.__width),
            Flooring(self.__flooring_type, self.__flooring_unit_cost),
        )

        return room
