"""
This module provides factory utilities for creating shapes. This includes
recording which Shape types are available.

"""

from __future__ import annotations

import copy

from shapes.shape import Shape
from shapes.circle import Circle
from shapes.square import Square
from shapes.triangle import (Triangle, RightTriangle, EquilateralTriangle)


_KNOWN_SHAPES = {
    "Triangle": Triangle,
    "Right Triangle": RightTriangle,
    "Equilateral Triangle": EquilateralTriangle,
    "Square": Square,
    "Circle": Circle
}  # _Dictionary_ of known shapes


def create(name: str) -> Shape:
    """
    Create a Shape

    Args:
        name: the shape to be created

    Returns:
        A shape with the specified name or null if no matching shape is found
    """

    if name in _KNOWN_SHAPES:
        return _KNOWN_SHAPES[name]()

    return None


def create_from_dimensions(name: str, values: list[float]) -> Shape:
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
        return _KNOWN_SHAPES[name](*values)

    return None


def is_known(name: str) -> bool:
    """
    Determine whether a given shape is known

    Args:
        name: the shape for which to query
    """

    return name in _KNOWN_SHAPES


def list_known() -> str:
    """
    Print a list of known Shapes
    """
    return "\n".join((f"  {name:}" for name in _KNOWN_SHAPES))


def number_known() -> int:
    """
    Determine the number of known Shapes
    """

    return len(_KNOWN_SHAPES)
