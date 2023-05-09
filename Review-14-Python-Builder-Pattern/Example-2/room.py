from __future__ import annotations

from dataclasses import dataclass


@dataclass
class Flooring:
    type_name: str = "Generic"
    unit_cost: float = 1.0


@dataclass
class DimensionSet:
    length: float = 1.0
    width: float = 1.0


class Room:
    def __init__(self, nme: str = "Generic"):
        self.__name = nme
        self.__dimensions = DimensionSet()
        self.__flooring = Flooring()

    @property
    def name(self):
        """
        Get the name using a property.

        @property must come before @...setter...
        """

        return self.__name

    def set_flooring(self, nme: str, unit_c: float):
        """
        Set the flooring.

        Args:
            nme: flooring type name
            unit_c: unit cost

        """

        self.__flooring.type_name = nme
        self.__flooring.unit_cost = unit_c

    def area(self) -> float:
        """
        Compute the area of flooring for a room.
        """

        return float(self.__dimensions.width) * self.__dimensions.length

    def flooring_cost(self) -> float:
        """
        Compute the flooring cost based on `self.area()` and unit cost.
        """

        return self.area() * self.__flooring.unit_cost

    def __str__(self) -> str:
        return "\n".join(
            (
                f"Room ({self.name})",
                "  {:<6}: {:>8.1f}".format("Length", self.__dimensions.length),
                "  {:<6}: {:>8.1f}".format("Width", self.__dimensions.width),
                "  {:<6}: {:>8.1f}".format("Area", self.area()),
                "",
                f"  Flooring  : {self.__flooring.type_name}",
                f"  Unit Cost : $ {self.__flooring.unit_cost:>8.2f}",
                "  Total Cost: $ {:>8.2f}".format(self.flooring_cost()),
                "\n",
            )
        )

    def __lt__(self, rhs) -> bool:
        if self.name == rhs.name:
            return self.area() < rhs.area()  # pylint caught the missing return

        return self.name < rhs.name

    def __eq__(self, rhs) -> bool:
        if not isinstance(rhs, Room):
            return False

        return self.name == (rhs.name) and self.area() == rhs.area()


class RoomBuilder:
    def __init__(self):
        self.__name = None
        self.__length = None
        self.__width = None
        self.__flooring = None

    def with_name(self, nme: str) -> RoomBuilder:
        """
        Set the name using the builder pattern.

        Args:
            nme: room name
        """

        self.__name = nme.strip()

        return self

    def with_flooring(self, nme: str, unit_c: float) -> RoomBuilder:
        """
        Set the Flooring using the builder pattern.

        Args
            nme: flooring type name
            unit_c: unit cost

        """

        flr = Flooring(nme, unit_c)
        self.__flooring = flr

        return self

    def with_dimensions(self, l: float, w: float) -> RoomBuilder:
        """
        Set the Flooring using the builder pattern.

        Args:
            l: length
            w: width

        """

        self.__length = l
        self.__width = w

        return self

    def validate_name(self) -> None:
        """
        Raise a Value Error if:
          1. Name was not set
          2. Name is the empty string
          3. Name has fewer than 3 characters
        """

        if not self.__name:
            raise ValueError("No name was set")

        if len(self.__name) < 3:
            raise ValueError("Name len('{self.__name}') < 3")

    def build(self) -> Room:
        self.validate_name()

        if not self.__flooring:
            raise ValueError("No flooring was set")

        if not self.__length:
            raise ValueError("No length was set")

        if not self.__width:
            raise ValueError("No width was set")

        room = Room(self.__name)
        room._Room__flooring = self.__flooring
        room._Room__dimensions = DimensionSet(self.__length, self.__width)

        return room
