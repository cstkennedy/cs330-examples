from __future__ import annotations

from typing import Callable
from triangle import (Point, Triangle)

MAX_NUM_TRIAS: int = 1_000_000

def default_check(tri: Triangle) -> bool:
    return True


class SurfaceGenerator:

    def __init__(starting_surface: List[Triangle], max_allowed_splits: int = 2):
        self.working_queue = starting_surface
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
                # Split the tri
                # Add the resulting triangles to new_queue
                pass
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
