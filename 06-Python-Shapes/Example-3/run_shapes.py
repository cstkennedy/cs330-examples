#! /usr/bin/env python3

# Programmer : Thomas J. Kennedy

import json
import pickle
import sys

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

    if len(sys.argv) < 2:
        print("No input file provided.")
        print("Usage: {:} input_file".format(*sys.argv))
        sys.exit(1)

    shapes_filename = sys.argv[1]

    print("-" * 80)

    for line in PROGRAM_HEADING:
        print(f"{line:^80}")

    print("-" * 80)

    #---------------------------------------------------------------------------
    # Examine the ShapeFactory
    #---------------------------------------------------------------------------
    print("~" * 38)
    print("{:^38}".format("Available Shapes"))
    print("~" * 38)
    print(ShapeFactory.list_known())
    print("-" * 38)
    print("{:>2} shapes available.".format(ShapeFactory.number_known()))
    print()

    #  make_circle = lambda attribs : Circle(*attribs)

    # print(ShapeFactory.create_from_dimensions("Circle", [4]))

    # The list needs to be intialized outside the "with" closure
    shapes = []  # Create an empty list (prefer `[]` over `list()`)

    # shapes_in = open(shapes_filename, "r")
    with open(shapes_filename, "r") as shapes_in:
        for line in shapes_in:
            line = line.strip()  # Strip leading/trailing whitespace
            # print(line)

            line = line.split(";")  # Split on ";"
            #  print(line)

            name, values = line  # Unpack the list
            #  print(name)
            #  print(values)
            values = values.strip()

            #  print(values)
            try:
                values = [float(val) for val in values.split()]
                #  print(values)
                shapes.append(ShapeFactory.create_from_dimensions(name, values))

            #  except ValueError as _err:
            except (ValueError, TypeError) as _err:
                print(f"Skipped shape \"{name:}\" due to malformed line.", file=sys.stderr)

    # Remove all `None` entries with a list comprehension
    shapes = [shp for shp in shapes if shp is not None]

    # Print all the shapes
    print("~" * 38)
    print("{:^38}".format("Display All Shapes"))
    print("~" * 38)

    for shp in shapes:
        print(shp)

    # for s in shapes:
    #     print(pickle.dumps(s, 0))

    # for shp in shapes:
    #     name = shp.__dict__["_name"]
    #     attribs = {key: s.__dict__[key] for key in s.__dict__ if key != "_name"}
    #     print(attribs)


if __name__ == "__main__":
    try:
        main()
    except FileNotFoundError as err:
        print(err)
