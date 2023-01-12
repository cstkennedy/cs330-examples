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
    
    @property
    def centroid(self):
        x = (vertex_0.x + vertex_1.x + vertex_2.x) / 3.0
        y = (vertex_0.y + vertex_1.y + vertex_2.y) / 3.0
        z = (vertex_0.z + vertex_1.z + vertex_2.z) / 3.0

        return Point(x, y, z)
