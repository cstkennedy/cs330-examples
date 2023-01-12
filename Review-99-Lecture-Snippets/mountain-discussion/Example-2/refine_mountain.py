from __future__ import annotations

import sys
import pprint as pp

from triangle import (Point, Triangle)
from refinement import SurfaceGenerator

def get_starting_mountain() -> list[Triangle]:
    pt_0 = Point(0.0, 0.0, 0.0 )
    pt_1 = Point(0.0, 100.0, 0.0)
    pt_2 = Point(100.0, 100.0, 0.0)
    pt_3 = Point(100.0, 0.0, 0.0)
    pt_4 = Point(50.0, 50.0, 75.0)

    return  [
        Triangle(pt_0, pt_4, pt_3),
        Triangle(pt_1, pt_4, pt_0),
        Triangle(pt_2, pt_4, pt_1),
        Triangle(pt_3, pt_4, pt_2)
    ]

def main() -> None:
    mountain = get_starting_mountain()

    #  pp.pprint(mountain, compact=False, indent=4, width=20)
    generator = SurfaceGenerator()
    #  generator.add_split_requirement()
    #  generator.add_split_requirement()
    #  generator.add_split_requirement()
    #  generator.add_split_requirement()

    while generator.has_more_work():
        generator.do_one_iteration()

    tris = generator.get_all_triangles() 
    

if __name__ == "__main__":
    main()
