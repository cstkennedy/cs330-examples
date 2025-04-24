"""
This module provides the Shape class and related constants which serve as the
base for other (specialized) shapes.
"""

from typing import Protocol

WIDTH_LABEL = 12  # Label Output Width
WIDTH_VALUE = 24  # Value Output Width

STR_FMT = f"{{:<{WIDTH_LABEL}}}:{{:>{WIDTH_VALUE}}}\n"
FPT_FMT = f"{{:<{WIDTH_LABEL}}}:{{:>{WIDTH_VALUE}.4f}}\n"


class Shape(Protocol):
    """
    Shape in a 2-D Cartesian Plane
    """

    @property
    def name(self) -> str:
        """
        Provide read-only access to the name attribute.

        Raises:
            NotImplemented Error if not overridden by subclass
        """

        raise NotImplementedError()

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
