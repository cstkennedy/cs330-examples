from __future__ import annotations

from triangle import (Point)

import math

def distance_between(pt_0: Point, pt_1: Point) -> float:
   x = pt_0.x - pt_1.x
   y = pt_0.y - pt_1.y
   z = pt_0.z - pt_1.z

   return math.sqrt(x**2 + y**2 + z**2)


def shift_to_origin(to_adjust: Point, offset: Point): -> Point:
    return Point(
        x = to_adjust.x - offset.x,
        y = to_adjust.y - offset.y,
        z = to_adjust.z - offset.z
    )

def shift(to_adjust: Point, offset: Point): -> Point:
    return Point(
        x = to_adjust.x + offset.x,
        y = to_adjust.y + offset.y,
        z = to_adjust.z + offset.z
    )

def cross_product_of(pt_a: Point, pt_b: Point) -> Point:
    return None

