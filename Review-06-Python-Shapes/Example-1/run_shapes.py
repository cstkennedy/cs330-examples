#! /usr/bin/env python3

# Programmer : Thomas Kennedy

# Note how I did not translate my
# utilities library that I wrote
# for the C++ and Java versions

# import utilities.Utilities;

from shapes import *


PROGRAM_HEADING = ["Objects & Inheritance: 2-D Shapes",
                   "Thomas J. Kennedy"]  # Program Title


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
        print("{:^80}".format(line))

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

    # Divide Output - Separate ShapeFactory Output
    print('~' * 80)
    print('~' * 80)

    # What happens when the number of shapes is non-trivial?
    #
    # Suppose we were to expand our Shape hierarchy to include
    # the following shapes:
    #   - Isosceles Triangle
    #   - Circle
    #   - Ellipse
    #   - Rectangle
    #   - Square
    #   - Rhombus
    #   - Parallelogram
    #   - Kite
    #   - Generalized Polygon
    #
    # How would we manage the addition of new Shapes?
    #
    # A common approach is to make use of the Factory Model.
    # This Model exists in a number of languages--e.g.:
    #   - C++
    #   - Java
    #   - Python
    #   - PHP
    #   - C#
    #
    # A class that contains static members is created.
    # As new classes are created, the Factory Class is
    # updated.
    #
    # In this example, our factory class is called ShapeFactory
    # The ShapeFactory could be designed as a singleton class.
    # Our ShapeFactory is simply a tracker--i.e., records are static
    # and will be updated manually at compile time.
    #
    # The Singleton Class implementation may be discussed at a
    # later date
    #

    # Examine the ShapeFactory
    print("~" * 38)
    print("{:^38}".format("Available Shapes"))
    print("~" * 38)

    # List the available shapes
    print(ShapeFactory.list_known())
    print("-" * 38)
    print("{:>2} shapes available.".format(ShapeFactory.number_known()))
    print()

    # Create 5 "Random" Shapes
    shapes = [ShapeFactory.create("Triangle"),
              ShapeFactory.create("Right Triangle"),
              ShapeFactory.create("Equilateral Triangle"),
              ShapeFactory.create("Square"),
              ShapeFactory.create("Circle"),
              ShapeFactory.create("1337 Haxor")]

    size = len(shapes)  # original size of the list

    # Remove all `None` entries with a list comprehension
    shapes = [s for s in shapes if s]

    # tempList = []
    # for s in shapes:
    #     #if s not None:
    #     if s:
    #         tempList.append(s)
    # shapes = tempList

    print("*" * 38)
    print("{:^38}".format("Shapes That Exist"))
    print("*" * 38)
    print("{:<24}: {:>4}".format("Original Size", size))
    print("{:<24}: {:>4}".format("Invalid Shapes", (size - len(shapes))))
    print("{:<24}: {:>4}".format("New Size", len(shapes)))
    print()

    # Print all the shapes
    print("~" * 38)
    print("{:^38}".format("Display All Shapes"))
    print("~" * 38)

    # for (Shape shp : shapes)
    for shp in shapes:
        print(shp)


if __name__ == "__main__":
    main()
