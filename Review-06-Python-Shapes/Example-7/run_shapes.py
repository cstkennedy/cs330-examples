#! /usr/bin/env python3

# Programmer : Thomas J. Kennedy


import logging
import sys
from typing import Generator, Optional, TextIO

from headings import BorderHeading, MultiLineBorderHeading
from shapes import shape_factory
from shapes.shape import Shape

PROGRAM_HEADING = MultiLineBorderHeading(
    content=(
        "Objects & Inheritance: 2-D Shapes",
        "Thomas J. Kennedy",
    ),
    width=80,
    symbol="-",
)

FACTORY_DESCRIPTION = "\n".join(
    (
        "~" * 38,
        "Available Shapes".center(38),
        "~" * 38,
        shape_factory.list_known(),
        "-" * 38,
        f"{shape_factory.number_known():>2} shapes available.\n",
    )
)


def read_shapes(shapes_in: TextIO) -> Generator[Optional[Shape], None, None]:
    """
    T.B.W.
    """

    for line in shapes_in:
        # Split on ";" and Strip leading/trailing whitespace
        # And Unpack the list
        name, values = [part.strip() for part in line.split(";")]

        values = values.strip()

        try:
            values = [float(val) for val in values.split()]
            shape = shape_factory.create_from_dimensions(name, values)

            yield shape

        except ValueError as _err:
            logging.warning(f'Skipped shape "{name:}" due to malformed line.')


def main():
    """
    The main function. In practice I could name this
    anything. The name main was selected purely
    out of familiarity.

    The "if __name__" line below determines what runs
    """

    if len(sys.argv) < 2:
        print("No input file provided.")
        print(f"Usage: {sys.argv[0]:} input_file")
        sys.exit(1)

    shapes_filename = sys.argv[1]

    print(PROGRAM_HEADING)
    print(FACTORY_DESCRIPTION)

    with open(shapes_filename, "r") as shapes_in:
        shapes = [shp for shp in read_shapes(shapes_in) if shp is not None]

    print(BorderHeading("Display All Shapes"))
    for shp in shapes:
        print(shp)

    print(BorderHeading("Display Largest Shape (Area)"))
    largest_shape = max(shapes, key=lambda shape: shape.area())
    print(largest_shape)

    print(BorderHeading("Display Smallest Shape (Perimeter)"))
    smallest_shape = min(shapes, key=lambda shape: shape.perimeter())
    print(smallest_shape)

    print(BorderHeading("Display Shapes Sorted by Name"))
    for shp in sorted(shapes, key=lambda shape: shape.name):
        print(shp)


if __name__ == "__main__":
    try:
        main()
    except FileNotFoundError as err:
        print(err)
