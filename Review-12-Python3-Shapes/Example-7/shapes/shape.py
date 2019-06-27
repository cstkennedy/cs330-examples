"""
This module provides the Shape class and related constants which serve as the
base for other (specialized) shapes.
"""


import copy


class Shape(object):
    """
    Shape in a 2-D Cartesian Plane
    """
    WIDTH_LABEL = 12  # Label Output Width
    WIDTH_VALUE = 24  # Value Output Width

    STR_FMT = "{:<" + str(WIDTH_LABEL) + "}:" + \
              "{:>" + str(WIDTH_VALUE) + "}\n"

    FPT_FMT = "{:<" + str(WIDTH_LABEL) + "}:" + \
              "{:>" + str(WIDTH_VALUE) + ".4f}\n"

    # @classmethod for static

    def __init__(self, name: str = "Shape"):
        """
        Shape Constructor
        :param: name the desired Shape name
        """

        self._name = name

    @property
    def name(self) -> str:
        """
        Provide read-only access to the name attribute.
        """

        return self._name

    @name.setter
    def name(self, _name: str):
        """
        Modify the name

        :param: _name new Shape name
        """

        self._name = _name

    def area(self) -> float:
        """
        Compute the area

        Raises:
            NotImplemented Error if not overridden by subclass
        """

        raise NotImplementedError()

    def perimeter(self) -> float:
        """
        Compute the perimeter

        Raises:
            NotImplemented Error if not overridden by subclass
        """

        raise NotImplementedError()

    def __deepcopy__(self, memo):
        """
        Return a new duplicate Shape
        """

        raise NotImplementedError()

    def __str__(self) -> str:
        """
        Print the shape
        """

        return Shape.STR_FMT.format("Name", self._name)
