from dataclasses import dataclass


class Flooring:
    def __init__(self, type_n: str = "Generic", unit_c: float = 1.0):
        self.type_name = type_n
        self.unit_cost = unit_c

    @property
    def unit_cost(self) -> float:
        return self.__unit_cost

    @unit_cost.setter
    def unit_cost(self, uc: float):
        self.__unit_cost = float(uc)


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

    def with_name(self, nme: str):
        """
        Set the name using the builder pattern.

        Args:
            nme: room name
        """

        self.__name = nme

        return self

    def with_flooring(self, nme: str, unit_c: float):
        """
        Set the Flooring using the builder pattern.

        Args
            nme: flooring type name
            unit_c: unit cost

        """

        self.__flooring.type_name = nme
        self.__flooring.unit_cost = unit_c

        return self

    def with_dimensions(self, l: float, w: float):
        """
        Set the Flooring using the builder pattern.

        Args:
            l: length
            w: width

        """

        self.__dimensions.length = l
        self.__dimensions.width = w

        return self

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

        return f"Room ({self.name})\n" \
             + "  {:<6}: {:>8.1f}\n".format("Length", float(self.__dimensions.length)) \
             + "  {:<6}: {:>8.1f}\n".format("Width", float(self.__dimensions.width)) \
             + "  {:<6}: {:>8.1f}\n".format("Area", self.area()) \
             + "\n" \
             + f"  Flooring  : {self.__flooring.type_name}\n" \
             + f"  Unit Cost : $ {self.__flooring.unit_cost:>8.2f}\n" \
             + "  Total Cost: $ {:>8.2f}\n".format(self.flooring_cost())

    def __lt__(self, rhs) -> bool:

        if self.name == rhs.name:
            return self.area() < rhs.area()  # pylint caught the missing return

        return self.name < rhs.name

    def __eq__(self, rhs) -> bool:
        if not isinstance(rhs, Room):
            return False

        return self.name == (rhs.name) and self.area() == rhs.area()
