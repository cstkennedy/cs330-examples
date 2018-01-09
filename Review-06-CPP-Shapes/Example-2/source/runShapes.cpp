// Thomas Kennedy
// Review Example: Inheritance and the Factory Model

#include <iostream>
#include <iomanip>

#include "utilities.h"
#include "shapeFactory.h"

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
 * Prune the Non-Existent--i.e., nullptr--shapes
 * from the array
 * 
 * @param shapes the array to prune
 * @param count the size of the array
 * 
 * @return the new array size
 */
int pruneNullPtr( Shape** &shapes, int count );

/**
 * This program does not accept command line
 * arguments
 */
int main()
{
    // --Erroneous Variable Declarations--
    //Shape shapes[4];               // Declare an Array of Shapes
    
    //Shape* shapes = new Shape[4] // Define an array of Shapes

    // --Corrected Variable Declarations--
    Shape**   shapes  = nullptr;

    // ShapeFactory Discussion
    int numShapes     = 0; // Number of shapes
    int size          = 0;

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
    size   = 6;
    shapes = new Shape*[ size ];

    shapes[0] = ShapeFactory::createShape( "Triangle" );
    shapes[1] = ShapeFactory::createShape( "Right Triangle" );
    shapes[2] = ShapeFactory::createShape( "Equilateral Triangle" );
    shapes[3] = ShapeFactory::createShape( "Square" );
    shapes[4] = ShapeFactory::createShape( "Circle" );
    shapes[5] = ShapeFactory::createShape( "1337 Haxor" );

    numShapes = pruneNullPtr( shapes, size );

    // numShapes is the size of the array after removal of nullptrs
    // size is the original size of the array

    printHeading( cout, "Shapes That Exist", 38, '~' );
    cout << left << setw( 24 ) << "Original Size"  << ": " 
         << left << setw( 4 )  << size              
         << "\n"
         << left << setw( 24 ) << "Invalid Shapes" << ": "
         << left << setw( 4 )  << ( size - numShapes )
         << "\n"
         << left << setw( 24 ) << "New Size"       << ": " 
         << left << setw( 4 )  << numShapes        << "\n";

    cout << "\n";

    // Print all the shapes
    printHeading( cout, "Display All Shapes", 38, '~' );

    for( int i = 0; i < numShapes; i++ ){
        cout << *(shapes[i]) << "\n";
    }

    // Cleanup
    for( int i = 0; i < numShapes; i++ ){
        delete shapes[i];
    }

    delete[] shapes;

    return 0;
}

/**
 *
 */
int pruneNullPtr( Shape** &shapes, int count )
{
    int nonNull = 0;

    // Copy all non null pointers to a new array
    Shape **pruned = new Shape*[ count ];

    for( int i = 0; i < count; i++ ){
        if( shapes[i] != nullptr ){
            pruned[nonNull] = shapes[i];
            nonNull++; 
        }
    }

    // Delete the original array
    delete[] shapes;

    // Create an a new array of size nonNull
    shapes = new Shape*[ nonNull ];

    // Copy the elements from the pruned array
    for( int i = 0; i < nonNull; i++ ){
        shapes[i] = pruned[i];
    }

    //delete the pruned array
    delete[] pruned;

    //return the new array size
    return nonNull;
}