from __future__ import annotations

from typing import (Callable, Tuple)
from triangle import (Point, Triangle)

MAX_NUM_TRIAS: int = 1_000_000

class SurfaceTriangle(Triangle):
    def __init__(self, v_0, v_1, v_2):
        super(Triangle, self).__init__(v_0, v_1, v_2)

    @property
    def side_a(self) -> Tuple[Point, Point]:
        return (self.vertex_0, self.vertex_1)

    @property
    def side_b(self) -> Tuple[Point, Point]:
        return (self.vertex_1, self.vertex_2)

    @property
    def side_c(self) -> Tuple[Point, Point]:
        return (self.vertex_2, self.vertex_0)

    @staticmethod
    def from(tri: Triangle):
        s_tri = SurfaceTriangle(
            v_0 = tri.vertex_0,
            v_1 = tri.vertex_1,
            v_2 = tri.vertex_2
        )

        return s_tri

def default_check(tri: Triangle) -> bool:
    return True


class SurfaceGenerator:

    def __init__(starting_surface: List[Triangle], max_allowed_splits: int = 2):
        self.working_queue = []
        for tri in starting_surface:
            self.working_queue.append(SurfaceTriangle.from(tri))

        self.finished_triangles = []
        self.split_criteria = []
        self.split_limit = max_allowed_splits

    def add_split_criteria(check: Callable[[Triangle], bool]):
        self.split_criteria.append(check)

    def has_more_work() -> bool:
        if len(self.working_queue) + len(self.finished_triangles) >= MAX_NUM_TRIAS:
            return False

        if len(self.working_queue) == 0:
            return False

        return True

    def  do_one_iteration() -> None:
        new_queue = []
        for tri in self.working_queue:
            if self.can_be_split(tri):
                # Split the triangle
                parent = tri
                centroid = tri.centroid
                offset = Point(0, 0, 0) # @todo replace with actual math
                shifted_centroid = shift(centroid, offset)

                tris = [
                    SurfaceTriangle(parent.vertex_0, parent.vertex_1, shifted_centroid),
                    SurfaceTriangle(parent.vertex_1, parent.vertex_2, shifted_centroid),
                    SurfaceTriangle(parent.vertex_0, shifted_centroid, parent.vertex_2)
                ]

                # Add the resulting triangles to new_queue
                new_queue += tris

            else:
                self.finished_triangles.append(tri)

        self.working_queue = new_queue

    def can_be_split(tri: Triangle) -> bool:
        #  if side ...
            #  return False

        #  if area...

        #  if height...

        #  if perimeter...

        for check in self.split_criteria:
            if not check(tri):
                return False

        return True
