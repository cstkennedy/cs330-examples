#! /usr/bin/env python3

# Programmer : Thomas J. Kennedy

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

    # Examine the ShapeFactory
    print("~" * 38)
    print("{:^38}".format("Available Shapes"))
    print("~" * 38)

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

    for shp in shapes:
        print(shp)


if __name__ == "__main__":
    main()
