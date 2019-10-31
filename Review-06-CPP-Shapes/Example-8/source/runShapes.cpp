// Thomas Kennedy
// Review Example: Inheritance and the Factory Model

#include <iostream>
#include <iomanip>
#include <fstream>
#include <vector>

#include <memory>
#include <iterator>
#include <algorithm>

#include "utilities.h"
#include "shapeFactory.h"
// #include "ShapeCollection.h" // No longer needed

/** @file */

using namespace std;
using namespace utilities;

const string PROGRAM_HEADING[] = {
    "Objects & Inheritance: 2-D Shapes",
    "Thomas J. Kennedy"
};  ///< Program Title

const int HEADING_LINES = 2;  ///< Number of lines in Program Heading

using ShapeCollection = std::vector<std::unique_ptr<Shape>>;

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

    // Print all the shapes
    printHeading(cout, "Display All Shapes", 38, '~');
    // cout << shapes;
    printShapes(cout, shapes);

    printHeading(cout, "Display Shape Names", 38, '~');
    // cout << shapes;
    printShapeNames(cout, shapes);

    printHeading(cout, "Display Largest Shape (Area)", 38, '~');
    ShapeCollection::const_iterator it;

    it = std::max_element(shapes.begin(), shapes.end(),
                          // [](const unique_ptr<Shape>& lhs, const unique_ptr<Shape>& rhs) {
                          [](const auto& lhs, const auto& rhs) {
                              return (lhs)->area() < (rhs)->area();
                          });
    cout << *(*it) << "\n";

    printHeading(cout, "Display Smallest Shape (Perimeter)", 38, '~');
    it = std::min_element(shapes.begin(), shapes.end(),
                          [](const auto& lhs, const auto& rhs) {
                              return (lhs)->perimeter() < (rhs)->perimeter();
                          });
    cout << *(*it) << "\n";

    printHeading(cout, "Display Sorted by Name", 38, '~');
    std::sort(shapes.begin(), shapes.end(),
              [](const auto& lhs, const auto& rhs) {
                  return (lhs)->name() < (rhs)->name();
              });
    printShapes(cout, shapes);

    return 0;
}

//------------------------------------------------------------------------------
ShapeCollection readShapes(std::istream& ins)
{
    ShapeCollection collection;
    /*
    Shape*          s;

    ins >> ws;

    while (ins >> s) {
        if (s != nullptr) {
            collection.emplace_back(s);
        }
        ins >> ws;
    }
    */
    // cout << collection << "\n";  // Not legal anymore

    std::istream_iterator<Shape*> it(ins);
    std::istream_iterator<Shape*> end;
    // Let us re-enact the Back-to-the-Future I Guitar Scene!
    for_each(it, end,
             [&collection](Shape* s) {
                  if (s != nullptr) {
                      collection.emplace_back(s);
                  }
             });

    return collection;
}

//------------------------------------------------------------------------------
void printShapes(std::ostream& outs, const ShapeCollection& toPrint)
{
    // for (const Shape* s : toPrint) {
    for (const std::unique_ptr<Shape>& s : toPrint) {
        // outs << s << "\n";  // oops
        outs << *s << "\n";
    }
}

//------------------------------------------------------------------------------
void printShapeNames(std::ostream& outs, const ShapeCollection& toPrint)
{
    for (const std::unique_ptr<Shape>& s : toPrint) {
        outs << s->name() << "\n";
    }
}
