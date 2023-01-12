from __future__ import annotations

import sys
from dataclasses import dataclass

@dataclass
class Point:
    """
    T.B.W.

    x - "right"
    y - "depth"
    z - "height"
    """
    x: float
    y: float
    z: float

@dataclass
class Triangle:
    vertex_0: Point
    vertex_1: Point
    vertex_2: Point
