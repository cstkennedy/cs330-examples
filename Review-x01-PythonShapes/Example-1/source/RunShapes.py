#! /usr/bin/env python3

# Programmer : Thomas Kennedy

# Note how I did not translate my
# utilities library that I wrote
# for the C++ and Java versions

#import utilities.Utilities;

from shapes import *


PROGRAM_HEADING = [
    "Objects & Inheritance: 2-D Shapes",
    "Thomas Kennedy"
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

    """
        // --Erroneous C++ Variable Declarations--
        // --Valid Java Variable Declarations--
        Shape               shape   = null; // Declare an "instance" of Shape
        Shape[]             shapes  = null; // Declare an Array of Shapes
        
        Triangle            tri     = null;
        RightTriangle       rhtTri  = null;
        EquilateralTriangle eqlTri  = null;

        // ShapeFactory Discussion
        int size = 0;
        
        // Print main program heading
        System.out.println(
            Utilities.projectHeading(PROGRAM_HEADING, Utilities.W_WIDTH)
        );
    """

    # Create one RightTriangle
    rhtTri = RightTriangle(1, 2)

    tri   = rhtTri  # Point tri to rhtTri
    shape = rhtTri  # Point shape to rhtTri

    # Is this a valid assignment?
    eqlTri = rhtTri
    # Yes, because Pyhton is loosely typed.

    print("{:^38}".format("Display a Right Triangle (rhtTri)"))
    print("-" * 38)
    print(tri)

    print("{:^38}".format("Display a Right Triangle (tri)"))
    print("-" * 38)
    print(tri)

    print("{:^38}".format("Display a Right Triangle (shape)"))
    print("-" * 38)
    print(shape)

    print('~' * 80)
    """  

    // Divide Output - Separate Right Triangle from Eql. Triangle
    System.out.println(
        Utilities.horizontalLine('~', Utilities.W_WIDTH)
    );
    System.out.println();

    rhtTri = null;

    // Create one Equilateral Triangle
    eqlTri = new EquilateralTriangle(8); 
    
    tri    = eqlTri; // Point tri to rhtTri    
    shape  = eqlTri; // Point shape to rhtTri

    System.out.println(
        Utilities.seperatedHeading("Display an Eql. Triangle (eqlTri)", 38, '-')
    );
    System.out.println(eqlTri);

    System.out.println(
        Utilities.seperatedHeading("Display an Eql. Triangle (tri)", 38, '-')
    );
    System.out.println(tri);

    System.out.println(
        Utilities.seperatedHeading("Display an Eql. Triangle (shape)", 38, '-')
    );
    System.out.println(shape); 
    """

    # Divide Output - Separate ShapeFactory Output
    print('~' * 80)


if __name__ == "__main__":
    main()
