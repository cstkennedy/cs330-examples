from __future__ import annotations

from triangle import (Point, Triangle)

MAX_NUM_TRIAS: int = 1_000_000

class SurfaceGenerator:

    def __init__(starting_surface: List[Triangle]):
        self.working_queue = starting_surface
        self.finished_triangles = []

    def has_more_work() -> bool:
        return False

    def  do_one_iteration() -> None:
        pass

