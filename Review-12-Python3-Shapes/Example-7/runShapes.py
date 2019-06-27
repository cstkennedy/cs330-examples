#! /usr/bin/env python3

# Programmer : Thomas J. Kennedy

# Note how I did not translate my
# utilities library that I wrote
# for the C++ and Java versions

# import utilities.Utilities;

from shapes import *

import pickle
import sys
import json

PROGRAM_HEADING = ["Objects & Inheritance: 2-D Shapes",
                   "Thomas J. Kennedy"]  # Program Title


def main():
    """
    The main function. In practice I could name this
    anything. The name main was selected purely
    out of familiarity.

    The "if __name__" line below determines what runs
    """

    if len(sys.argv) < 2:
        print("No input file provided.")
        print("Usage: {:} input_file".format(*sys.argv))
        exit(1)

    shapes_filename = sys.argv[1]

    # Print Program Heading
    print("-" * 80)

    for line in PROGRAM_HEADING:
        print("{:^80}".format(line))

    print("-" * 80)

    # Examine the shape_factory
    print("~" * 38)
    print("{:^38}".format("Available Shapes"))
    print("~" * 38)

    # List the available shapes
    print(shape_factory.list_known())
    print("-" * 38)
    print("{:>2} shapes available.".format(shape_factory.number_known()))
    print()

    # The list needs to be intialzed outside the "with" closure
    shapes = list()

    with open(shapes_filename, "r") as shapes_in:
        for line in shapes_in:
            # Split on ";" and Strip leading/trailing whitespace
            # And Unpack the list
            name, values = [part.strip() for part in line.split(";")]

            values = json.loads(values)

            shapes.append(shape_factory.create_from_dictionary(name, values))

    # Remove all `None` entries with a list comprehension
    shapes = [s for s in shapes if s is not None]

    # Print all the shapes
    print("~" * 38)
    print("{:^38}".format("Display All Shapes"))
    print("~" * 38)

    for s in shapes:
        print(s)

    out_filename = "coolPickles.dat"

    with open(out_filename, "wb") as pickle_file:
        # LOL Nope
        # for s in shapes:
        #     pickle.dump(s, pickle_file)

        # One line, full data structure
        pickle.dump(shapes, pickle_file)

    with open(out_filename, "rb") as pickle_file:
        rebuilt_shapes = pickle.load(pickle_file)

    # Print all the rebuilt shapes
    print("~" * 38)
    print("{:^38}".format("Display Re-Built Shapes"))
    print("~" * 38)

    for s in rebuilt_shapes:
        print(s)

    print("~" * 38)
    print("{:^38}".format("Display Largest Shape (Area)"))
    print("~" * 38)

    largest_shape = max(rebuilt_shapes, key=lambda shape: shape.area())
    print(largest_shape)

    print("~" * 38)
    print("{:^38}".format("Display Smallest Shape (Perimeter)"))
    print("~" * 38)

    smallest_shape = min(rebuilt_shapes, key=lambda shape: shape.perimeter())
    print(smallest_shape)


if __name__ == "__main__":
    try:
        main()
    except FileNotFoundError as e:
        print(e)
