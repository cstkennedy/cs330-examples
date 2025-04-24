#! /usr/bin/env python3

# Programmer : Thomas J. Kennedy

# Note how I did not translate my
# utilities library that I wrote
# for the C++ and Java versions

# import utilities.Utilities;

from shapes import *

PROGRAM_HEADING = ("Objects & Inheritance: 2-D Shapes",
                   "Thomas J. Kennedy")  # Program Title


def main():
    """
    The main function. In practice I could name this
    anything. The name main was selected purely
    out of familiarity.

    The "if __name__" line below determines what runs
    """

    # Print Program Heading
    print("-" * 80)
    for line in PROGRAM_HEADING:
        print(f"{line:^80}")
    print("-" * 80)

    """
    # --Erroneous C++ Variable Declarations--
    # --Valid Java Variable Declarations--
    Shape               shape   = null; # Declare an "instance" of Shape
    Shape[]             shapes  = null; # Declare an Array of Shapes

    Triangle            tri     = null;
    RightTriangle       rht_tri  = null;
    EquilateralTriangle eql_tri  = null;

    # ShapeFactory Discussion
    int size = 0;
    """

    # Create one RightTriangle
    rht_tri = RightTriangle(1, 2)

    tri = rht_tri  # Point tri to rht_tri
    shape = rht_tri  # Point shape to rht_tri

    # Is this a valid assignment?
    eql_tri = rht_tri
    # Yes, because Python is loosely typed.

    print("{:^38}".format("Display a Right Triangle (rht_tri)"))
    print("-" * 38)
    print(rht_tri)

    print("{:^38}".format("Display a Right Triangle (tri)"))
    print("-" * 38)
    print(tri)

    print("{:^38}".format("Display a Right Triangle (shape)"))
    print("-" * 38)
    print(shape)

    print('~' * 80)

    # Create one Equilateral Triangle
    eql_tri = EquilateralTriangle(8)

    tri = eql_tri  # Point tri to rht_tri
    shape = eql_tri  # Point shape to rht_tri

    print("{:^38}".format("Display an Eql. Triangle (eql_tri)"))
    print("-" * 38)
    print(eql_tri)

    print("{:^38}".format("Display an Eql. Triangle (tri)"))
    print("-" * 38)
    print(tri)

    print("{:^38}".format("Display an Eql. Triangle (shape)"))
    print("-" * 38)
    print(shape)


if __name__ == "__main__":
    main()
