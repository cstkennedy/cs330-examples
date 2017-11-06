// Thomas Kennedy
// Review Example: Inheritance

#include <iostream>
#include <iomanip>

#include "utilities.h"

#include "shape.h"
#include "triangle.h"
#include "rightTriangle.h"
#include "equilateralTriangle.h"

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
    // --Erroneous Variable Declarations--
    //Shape shape;                   // Declare an instance of Shape
    //Shape shapes[4];               // Declare an Array of Shapes
    
    //Shape* shapes = new Shape[4] // Define an array of Shapes

    // --Corrected Variable Declarations--
    Shape*    shape   = nullptr;
    Shape**   shapes  = nullptr; // Not used in Example 1

    Triangle* tri     = nullptr;
    RhtTri*   rhtTri  = nullptr;
    EqlTri*   eqlTri  = nullptr;

    // Set formatting
    cout.precision( 4 );
    cout.setf( ios::fixed );

    printProjectHeading( cout, PROGRAM_HEADING, HEADING_LINES );

    // Create one RightTriangle
    // & one EquilateralTriangle
    //rhtTri  = new RightTriangle(1, 2);
    rhtTri  = new RhtTri(1, 2);
    
    tri   = rhtTri; // Point tri to rhtTri    
    shape = rhtTri; // Point shape to rhtTri

    /*
    // Is this a valid assignment?
    eqlTri = rhtTri;
    */

    printSeperatedHeading( cout, "Display a Right Triangle (rhtTri)", 38 );
    rhtTri->display( cout );
    cout << "\n";

    printSeperatedHeading( cout, "Display a Right Triangle (tri)", 38 );
    tri->display( cout );
    cout << "\n";

    printSeperatedHeading( cout, "Display a Right Triangle (shape)", 38 );
    shape->display( cout );
    cout << "\n";

    //Force the use of Triangle::display
    printSeperatedHeading( cout, "Print a Right Triangle as a Triangle", 38 );
    rhtTri->Triangle::display( cout );
    cout << "\n";

    //Force the use of Shape::display
    printSeperatedHeading( cout, "Print a Right Triangle as a Shape", 38 );
    rhtTri->Shape::display( cout );
    cout << "\n";

    /*
    // Force the use of Triangle::area
    // Display the differences in calculation

    shape = new RightTriangle( 1.412, 2.824 );

    // Increase the displayed precision to 16
    cout.precision( 16 );

    cout << ( (RightTriangle*) shape )->area()           << "\n";
    cout << ( (RightTriangle*) shape )->Triangle::area() << "\n";
    */

    // Divide Output - Separate Right Triangle from Eql. Triangle
    printHorizontalLine( cout, '~', W_WIDTH );
    cout << "\n";

    // Pointer Review - Why not all three pointers?
    delete rhtTri; // Deallocate the RightTriangle Instance
    //delete tri;   // Why can tri not be deleted?
    //delete shape; // Why can shape not be deleted?

    rhtTri = nullptr;

    // Create one Equilateral Triangle
    //eqlTri  = new EquilateralTriangle(8); 
    eqlTri  = new EqlTri(8); 
    
    tri   = eqlTri; // Point tri to rhtTri    
    shape = eqlTri; // Point shape to rhtTri

    printSeperatedHeading( cout, "Display an Eql. Triangle (eqlTri)", 38 );
    eqlTri->display( cout );
    cout << "\n";

    printSeperatedHeading( cout, "Display an Eql. Triangle (tri)", 38 );
    tri->display( cout );
    cout << "\n";

    printSeperatedHeading( cout, "Display an Eql. Triangle (shape)", 38 );
    shape->display( cout );
    cout << "\n";

    //Force the use of Triangle::display
    printSeperatedHeading( cout, "Print an Eql. Triangle as a Triangle", 38 );
    eqlTri->Triangle::display( cout );
    cout << "\n";

    //Force the use of Shape::display
    printSeperatedHeading( cout, "Print an Eql. Triangle as a Shape", 38 );
    eqlTri->Shape::display( cout );
    cout << "\n";    

    cout << "\n";

    // Pointer Review
    delete eqlTri; // Deallocate the EquilateralTriangle Instance
    //delete tri;   // Why can tri not be deleted?
    //delete shape; // Why can shape not be deleted?

    // Divide Output - Separate ShapeFactory Output
    printHorizontalLine( cout, '~', W_WIDTH );
    cout << "\n";
    cout << "\n";

    return 0;
}
