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

    out_filename = "coolPickles.dat"

    with open(out_filename, "rb") as pickle_file:
        rebuilt_shapes = pickle.load(pickle_file)

    # Print all the rebuilt shapes
    print("~" * 38)
    print("{:^38}".format("Display Re-Built Shapes"))
    print("~" * 38)

    for shp in rebuilt_shapes:
        print(shp)


if __name__ == "__main__":
    try:
        main()
    except FileNotFoundError as err:
        print(err)
