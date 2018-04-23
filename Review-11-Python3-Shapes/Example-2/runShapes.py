#! /usr/bin/env python3

# Programmer : Thomas Kennedy

# Note how I did not translate my
# utilities library that I wrote
# for the C++ and Java versions

# import utilities.Utilities;

from shapes import *


PROGRAM_HEADING = [
    "Objects & Inheritance: 2-D Shapes",
    "Thomas J. Kennedy"
]  # Program Title


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
    print("*" * 38)
    print("{:^38}".format("Available Shapes"))
    print("*" * 38)

    # List the available shapes
    print(ShapeFactory.listKnown())
    print("-" * 38)
    print("{:>2} shapes available.".format(ShapeFactory.numberKnown()))

    print()

    # Create 5 "Random" Shapes
    size = 6  # original size of the list
    shapes = [
        ShapeFactory.createShape("Triangle"),
        ShapeFactory.createShape("Right Triangle"),
        ShapeFactory.createShape("Equilateral Triangle"),
        ShapeFactory.createShape("Square"),
        ShapeFactory.createShape("Circle"),
        ShapeFactory.createShape("1337 Haxor")
    ]

    # Remove all `None` entries with a list comprehension
    shapes = [s for s in shapes if s]

    print("*" * 38)
    print("{:^38}".format("Shapes That Exist"))
    print("*" * 38)
    print("{:<24}: {:>4}".format("Original Size", size))
    print("{:<24}: {:>4}".format("Invalid Shapes", (size - len(shapes))))
    print("{:<24}: {:>4}".format("New Size", len(shapes)))

    print()

    # Print all the shapes
    print("*" * 38)
    print("{:^38}".format("Display All Shapes"))
    print("*" * 38)

    for s in shapes:
        print(s)


if __name__ == "__main__":
    main()
