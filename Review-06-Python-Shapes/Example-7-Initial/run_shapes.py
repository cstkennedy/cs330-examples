#! /usr/bin/env python3

# Programmer : Thomas J. Kennedy

import sys

import shapes.shape_factory as shape_factory

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
        #  print("Usage: {:} input_file".format(*sys.argv))
        print(f"Usage: {sys.argv[0]:} input_file")
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
    print(shape_factory.list_known())
    print("-" * 38)
    print(f"{shape_factory.number_known():>2} shapes available.")
    print()

    # The list needs to be intialized outside the "with" closure
    shapes = []  # Create an empty list (prefer `[]` over `list()`)

    with open(shapes_filename, "r") as shapes_in:
        for line in shapes_in:
            # Split on ";" and Strip leading/trailing whitespace
            # And Unpack the list
            name, values = [part.strip() for part in line.split(";")]

            values = values.strip()

            try:
                values = [float(val) for val in values.split()]
                shapes.append(shape_factory.create_from_dimensions(name, values))

            except ValueError as _err:
                print(f"Skipped shape \"{name:}\" due to malformed line.", file=sys.stderr)

    # Remove all `None` entries with a list comprehension
    shapes = [shp for shp in shapes if shp is not None]

    # Print all the shapes
    print("~" * 38)
    print("{:^38}".format("Display All Shapes"))
    print("~" * 38)

    for shp in shapes:
        print(shp)

    print("~" * 38)
    print("{:^38}".format("Display Largest Shape (Area)"))
    print("~" * 38)

    largest_shape = max(shapes, key=lambda shape: shape.area())
    print(largest_shape)

    print("~" * 38)
    print("{:^38}".format("Display Smallest Shape (Perimeter)"))
    print("~" * 38)

    smallest_shape = min(shapes, key=lambda shape: shape.perimeter())
    print(smallest_shape)

    sorted_shapes = sorted(shapes, key=lambda shape: shape.name)

    for shp in sorted_shapes:
        print(shp)


if __name__ == "__main__":
    try:
        main()
    except FileNotFoundError as err:
        print(err)
