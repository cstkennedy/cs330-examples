#! /usr/bin/env python3

# Programmer : Thomas Kennedy

# Note how I did not translate my
# utilities library that I wrote
# for the C++ and Java versions

# import utilities.Utilities;

from shapes import *

import pickle
import sys
import json

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

    if len(sys.argv) < 2:
        print("No input file provided.")
        print("Usage: {:} input_file".format(*sys.argv))
        exit(1)

    shapesFilename = sys.argv[1]

    # Print Program Heading
    print("-" * 80)

    for line in PROGRAM_HEADING:
        print("{:^80}".format(line))

    print("-" * 80)

    # Examine the ShapeFactory
    print("~" * 38)
    print("{:^38}".format("Available Shapes"))
    print("~" * 38)

    # List the available shapes
    print(ShapeFactory.listKnown())
    print("-" * 38)
    print("{:>2} shapes available.".format(ShapeFactory.numberKnown()))
    print()

    # The list needs to be intialzed outside the "with" closure
    shapes = list()

    with open(shapesFilename, "r") as shapesIn:
        for line in shapesIn:
            # Split on ";" and Strip leading/trailing whitespace
            # And Unpack the list
            name, values = [part.strip() for part in line.split(";")]

            values = json.loads(values)

            shapes.append(ShapeFactory.createFromDictionary(name, values))

    # Remove all `None` entries with a list comprehension
    shapes = [s for s in shapes if s is not None]

    # Print all the shapes
    print("~" * 38)
    print("{:^38}".format("Display All Shapes"))
    print("~" * 38)

    for s in shapes:
        print(s)

    outFilename = "coolPickles.dat"

    with open(outFilename, "wb") as pickleF:
        # LOL Nope
        # for s in shapes:
        #     pickle.dump(s, pickleF)

        # One line, full data structure
        pickle.dump(shapes, pickleF)

    # reBuiltShapes = None

    with open(outFilename, "rb") as pickleF:
        reBuiltShapes = pickle.load(pickleF)

    # Print all the reBuilt shapes
    print("~" * 38)
    print("{:^38}".format("Display Re-Built Shapes"))
    print("~" * 38)

    for s in reBuiltShapes:
        print(s)

    # for s in shapes:
    #     name = s.__dict__["_name"]
    #     attribs = {key: s.__dict__[key] for key in s.__dict__ if key != "_name"}
    #     print(attribs)

if __name__ == "__main__":
    try:
        main()
    except FileNotFoundError as e:
        print(e)
