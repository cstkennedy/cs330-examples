// Thomas Kennedy
// Review Example: Inheritance and the Factory Model

#include <iostream>
#include <iomanip>
#include <fstream>

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
 * Read shapes from an input stream
 * and construct a `ShapeCollection` object
 */
ShapeCollection readShapes(std::istream& ins);

/**
 * Print shapes from a `ShapeCollection` to a
 * specified output stream
 */
void printShapes(std::ostream& outs, const ShapeCollection& toPrint);

/**
 * Print shape names for all `Shape`s in a `ShapeCollection` to a
 * specified output stream
 */
void printShapeNames(std::ostream& outs, const ShapeCollection& toPrint);

/** 
 * Find the largest `Shape` (by area) in a `ShapeCollection`
 *
 * @return an iterator at the position of the largest `Shape`
 */
ShapeCollection::const_iterator findLargestShapeByArea(const ShapeCollection& collection);

/**
 * This program accepts command line
 * arguments
 */
int main(int argc, char** argv)
{
    if (argc < 2) {
        cout << "No input file provided." << "\n";
        cout << "Usage: " << argv[0] << " input_file" << "\n";
        return 1;
    }

    ifstream shapesFile(argv[1]);

    if (!shapesFile) {
        cout << "Error: " << argv[1] << "could not be opened" << "\n";
        return 2;
    }

    // Set formatting
    cout.precision(4);
    cout.setf(ios::fixed);

    printProjectHeading(cout, PROGRAM_HEADING, HEADING_LINES);

    // Examine the ShapeFactory
    printHeading(cout, "Available Shapes", 38, '~');

    // List the available shapes
    ShapeFactory::listKnown(cout);
    printHorizontalLine(cout, '-', 38);
    cout << right << setw(2) 
         << ShapeFactory::numberKnown() 
         << " shapes available." 
         << "\n";    

    cout << "\n";

    // Create 5 "Random" Shapes
    ShapeCollection shapes = readShapes(shapesFile);

    // Demonstrate assertions (review)
    //shapes.addShape(ShapeFactory::createShape("1337 Haxor"));

    // Print all the shapes
    printHeading(cout, "Display All Shapes", 38, '~');
    //cout << shapes;
    printShapes(cout, shapes);

    printHeading(cout, "Display Shape Names", 38, '~');
    //cout << shapes;
    printShapeNames(cout, shapes);

    printHeading(cout, "Display Largest Shape (Area)", 38, '~');

    ShapeCollection::const_iterator it = findLargestShapeByArea(shapes);
    Shape* largestShape = *it; 

    //cout << largestShape << "\n"; // oops again
    cout << *largestShape << "\n";
    //cout << *(*it) << "\n"; // skip the temporary Shape*

    return 0;
}

/**
 *
 */
ShapeCollection readShapes(std::istream& ins)
{
    ShapeCollection collection;
    Shape*          s;

    ins >> ws;

    while(ins >> s) {
        if (s != nullptr) {
            collection.addShape(s);
        }
        ins >> ws;
    }

    cout << collection << "\n";

    return collection;
}

/**
 *
 */
void printShapes(std::ostream& outs, const ShapeCollection& toPrint)
{
    for(const Shape* s : toPrint) {
        //outs << s << "\n"; // oops
        outs << *s << "\n";
    }
}

/**
 *
 */
void printShapeNames(std::ostream& outs, const ShapeCollection& toPrint)
{
    for(const Shape* s : toPrint) {
        outs << s->name() << "\n";
    }
}

ShapeCollection::const_iterator findLargestShapeByArea(const ShapeCollection& collection)
{
    ShapeCollection::const_iterator it           = collection.begin();
    ShapeCollection::const_iterator largestShape = it;

    // The first shape is the largest
    // until I look at more
    largestShape = it;

    // Review pre increment
    while(++it != collection.end()) {
        //std::cerr << "\n" << *(*it) << "\n"; // debugging

        if ((*it)->area() > (*largestShape)->area()) {
            largestShape = it;
        }
    }

    return largestShape;    
}