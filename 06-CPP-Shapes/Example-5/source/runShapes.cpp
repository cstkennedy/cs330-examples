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
};  ///< Program Title

const int HEADING_LINES = 2;  ///< Number of lines in Program Heading

/**
 * Read shapes from an input stream
 * and construct a `ShapeCollection` object
 */
ShapeCollection readShapes(std::istream& ins);

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
        cout << "Error: " << argv[1] << " could not be opened" << "\n";
        return 2;
    }

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
    ShapeCollection shapes = readShapes(shapesFile);

    // Print all the shapes
    printHeading(cout, "Display All Shapes", 38, '~');
    for (const Shape* shape : shapes) {
        cout << *shape << "\n";
    }

    return 0;
}

//------------------------------------------------------------------------------
ShapeCollection readShapes(std::istream& ins)
{
    ShapeCollection collection;
    Shape* s = nullptr;

    ins >> ws;

    while (ins >> s) {
        if (s != nullptr) {
            collection.addShape(s);
        }
        ins >> ws;
    }

    return collection;
}
