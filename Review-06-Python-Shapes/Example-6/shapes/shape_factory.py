"""
This module provides factory utilities for creating shapes. This includes
recording which Shape types are available.

"""

import copy

from shapes.circle import Circle
from shapes.square import Square
from shapes.triangle import (Triangle, RightTriangle, EquilateralTriangle)


_KNOWN_SHAPES = {
    "Triangle": (
        Triangle(),
        lambda a, b, c: Triangle(a, b, c)
    ),
    "Right Triangle": (
        RightTriangle(),
        lambda base, height: RightTriangle(base, height)
    ),
    "Equilateral Triangle": (
        EquilateralTriangle(),
        lambda side: EquilateralTriangle(side)
    ),
    "Square": (
        Square(),
        lambda side: Square(side)
    ),
    "Circle": (
        Circle(),
        lambda radius: Circle(radius)
    )
}  # _Dictionary_ of known shapes


def create(name):
    """
    Create a Shape

    Args:
        name: the shape to be created

    Returns:
        A shape with the specified name or null if no matching shape is found
    """

    if name in _KNOWN_SHAPES:
        return copy.deepcopy(_KNOWN_SHAPES[name][0])

    return None


def create_from_dimensions(name, values):
    """
    Create a Shape

    Args:
        name: the shape to be created

        values: dictionary of values corresponding to the data needed
            to inialize a shape

    Returns:
        A shape with the specified name or null if no matching shape is found
    """

    if name in _KNOWN_SHAPES:
        return _KNOWN_SHAPES[name][1](*values)

    return None


def is_known(name):
    """
    Determine whether a given shape is known

    Args:
        name: the shape for which to query
    """

    return name in _KNOWN_SHAPES


def list_known():
    """
    Print a list of known Shapes
    """
    return "\n".join((f"  {name:}" for name in _KNOWN_SHAPES))


def number_known():
    """
    Determine the number of known Shapes
    """

    return len(_KNOWN_SHAPES)
