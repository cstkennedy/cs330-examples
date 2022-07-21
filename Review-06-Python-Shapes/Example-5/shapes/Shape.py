"""
This module provides the Shape class and related constants which serve as the
base for other (specialized) shapes.
"""

import abc

WIDTH_LABEL = 12  # Label Output Width
WIDTH_VALUE = 24  # Value Output Width

STR_FMT = f"{{:<{WIDTH_LABEL}}}:{{:>{WIDTH_VALUE}}}\n"
FPT_FMT = f"{{:<{WIDTH_LABEL}}}:{{:>{WIDTH_VALUE}.4f}}\n"


class Shape(metaclass=abc.ABCMeta):
    """
    Shape in a 2-D Cartesian Plane
    """

    # @classmethod for static

    def __init__(self, name="Shape"):
        """
        Shape Constructor
        :param: name the desired Shape name
        """

        self._name = name

    @property
    def name(self):
        return self._name

    @name.setter
    def name(self, _name):
        """
        Modify the name

        :param: _name new Shape name
        """

        self._name = _name

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

        Raises:
            NotImplemented Error if not overridden by subclass
        """

        raise NotImplementedError()

    @abc.abstractmethod
    def __str__(self) -> str:
        """
        Print the shape

        Raises:
            NotImplemented Error if not overridden by subclass
        """

        return STR_FMT.format("Name", self.name)
