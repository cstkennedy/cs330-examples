// Thomas Kennedy
// Review Example: Inheritance and the Factory Model

#include <iostream>
#include <iomanip>

#include "utilities.h"
#include "shapeFactory.h"
#include "ShapeCollection.h"

/** @file */

using namespace std;
using namespace utilities;

const string PROGRAM_HEADING[] = {
    "Objects & Inheritance: 2-D Shapes",
    "Thomas J. Kennedy"
};  ///< Program Title

const int HEADING_LINES = 2;  ///< Number of lines in Program Heading

typedef RightTriangle       RhtTri;  ///< Convenient shorthand for RightTriangle
typedef EquilateralTriangle EqlTri;  ///< Convenient shorthand for EquilateralTriangle

/**
 * This program does not accept command line
 * arguments
 */
int main()
{
    // Set formatting
    cout.precision(4);
    cout.setf(ios::fixed);

    printProjectHeading(cout, PROGRAM_HEADING, HEADING_LINES);

    // Examine the ShapeFactory
    printHeading(cout, "Available Shapes", 38, '~');
    ShapeFactory::listKnown(cout);
    printHorizontalLine(cout, '-', 38);
    cout << right << setw(2)
         << ShapeFactory::numberKnown()
         << " shapes available."
         << "\n";

    cout << "\n";

    // Create 5 "Random" Shapes
    ShapeCollection shapes(5);

    shapes.addShape(ShapeFactory::createShape("Triangle"));
    shapes.addShape(ShapeFactory::createShape("Right Triangle"));
    shapes.addShape(ShapeFactory::createShape("Equilateral Triangle"));
    shapes.addShape(ShapeFactory::createShape("Square"));
    shapes.addShape(ShapeFactory::createShape("Circle"));
    shapes.addShape(ShapeFactory::createShape("1337 Haxor"));

    // Print all the shapes
    printHeading(cout, "Display All Shapes", 38, '~');
    for (const Shape* shape : shapes) {
        cout << *shape << "\n";
    }

    return 0;
}
