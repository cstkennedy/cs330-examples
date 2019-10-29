import abc

"""
This module provides the Shape class and related constants which serve as the
base for other (specialized) shapes.
"""

WIDTH_LABEL = 12  # Label Output Width
WIDTH_VALUE = 24  # Value Output Width

STR_FMT = "{:<" + str(WIDTH_LABEL) + "}:" + \
          "{:>" + str(WIDTH_VALUE) + "}\n"

FPT_FMT = "{:<" + str(WIDTH_LABEL) + "}:" + \
          "{:>" + str(WIDTH_VALUE) + ".4f}\n"


class Shape(metaclass=abc.ABCMeta):
    """
    Shape in a 2-D Cartesian Plane
    """

    # @classmethod for static

    def __init__(self):
        """
        Shape Constructor
        """

        pass

    @property
    @abc.abstractmethod
    def name(self) -> str:
        """
        Provide read-only access to the name attribute.

        Raises:
            NotImplemented Error if not overridden by subclass
        """

        raise NotImplementedError()

    @abc.abstractmethod
    def area(self) -> float:
        """
        Compute the area

        Raises:
            NotImplemented Error if not overridden by subclass
        """

        raise NotImplementedError()

    @abc.abstractmethod
    def perimeter(self) -> float:
        """
        Compute the perimeter

        Raises:
            NotImplemented Error if not overridden by subclass
        """

        raise NotImplementedError()

    @abc.abstractmethod
    def __deepcopy__(self, memo):
        """
        Return a new duplicate Shape
        """

        raise NotImplementedError()

    @abc.abstractmethod
    def __str__(self) -> str:
        """
        Print the shape
        """

        return STR_FMT.format("Name", self.name)
