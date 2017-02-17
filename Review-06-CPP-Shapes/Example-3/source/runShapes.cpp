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
}; ///< Program Title 

const int HEADING_LINES = 2; ///< Number of lines in Program Heading

typedef RightTriangle       RhtTri; ///< Convenient shorthand for RightTriangle
typedef EquilateralTriangle EqlTri; ///< Convenient shorthand for EquilateralTriangle

/**
 * This program does not accept command line
 * arguments
 */
int main()
{
    // Set formatting
    cout.precision( 4 );
    cout.setf( ios::fixed );

    printProjectHeading( cout, PROGRAM_HEADING, HEADING_LINES );

    /*
    * What happens when the number of shapes is non-trivial?
    *
    * Suppose we were to expand our Shape hierarchy to include
    * the following shapes:
    *   - Isosceles Triangle 
    *   - Circle
    *   - Ellipse
    *   - Rectangle
    *   - Square
    *   - Rhombus
    *   - Parallelogram
    *   - Kite
    *   - Generalized Polygon
    *
    * How would we manage the addition of new Shapes?
    */

    /*
    * A common approach is to make use of the Factory Model.
    * This Model exists in a number of languages--e.g.:
    *   - C++
    *   - Java
    *   - Python
    *   - PHP
    *   - C#
    */

    /*
    * A class that contains static members is created.
    * As new classes are created, the Factory Class is
    * updated.
    *
    * In this example, our factory class is called ShapeFactory
    * The ShapeFactory could be designed as a singleton class.
    * Our ShapeFactory is simply a tracker--i.e., records are static
    * and will be updated manually at compile time.
    *
    * The Singleton Class implementation may be discussed at a
    * later date
    */

    // Examine the ShapeFactory
    printHeading( cout, "Available Shapes", 38, '~' );

    // List the available shapes
    ShapeFactory::listKnown( cout );
    printHorizontalLine( cout, '-', 38 );
    cout << right << setw( 2 ) << ShapeFactory::numberKnown() << " shapes available." << "\n";    

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
    printHeading( cout, "Display All Shapes", 38, '~' );
    cout << shapes;

    return 0;
}