// Thomas Kennedy
// Review Example: Inheritance and the Factory Model

#include <iostream>
#include <iomanip>
#include <fstream>
#include <vector>
#include <array>

#include <memory>
#include <iterator>
#include <algorithm>

#include "utilities.h"
#include "shapeFactory.h"
// #include "ShapeCollection.h" // No longer needed

/** @file */

using namespace std;
using namespace utilities;

const std::array<string_view, 2> PROGRAM_HEADING = {
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

    printProjectHeading(cout, PROGRAM_HEADING);

    // Examine the ShapeFactory
    printHeading<'~', 38>(cout, "Available Shapes");

    // List the available shapes
    ShapeFactory::listKnown(cout);
    printHorizontalLine<'-', 38>(cout);
    cout << right << setw(2)
         << ShapeFactory::numberKnown()
         << " shapes available."
         << "\n";

    cout << "\n";

    // Create 5 "Random" Shapes
    ShapeCollection shapes = readShapes(shapesFile);

    // Print all the shapes
    printHeading<'~', 38>(cout, "Display All Shapes");
    printShapes(cout, shapes);

    printHeading<'~', 38>(cout, "Display Shape Names");
    printShapeNames(cout, shapes);

    printHeading<'~', 38>(cout, "Display Largest Shape (Area)");
    ShapeCollection::const_iterator it;

    it = std::max_element(shapes.begin(), shapes.end(),
                          [](const auto& lhs, const auto& rhs) {
                              return (lhs)->area() < (rhs)->area();
                          });
    cout << *(*it) << "\n";

    printHeading<'~', 38>(cout, "Display Smallest Shape (Perimeter)");
    it = std::min_element(shapes.begin(), shapes.end(),
                          [](const auto& lhs, const auto& rhs) {
                              return (lhs)->perimeter() < (rhs)->perimeter();
                          });
    cout << *(*it) << "\n";

    printHeading<'~', 38>(cout, "Display Sorted by Name");
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
    for (const std::unique_ptr<Shape>& s : toPrint) {
        // outs << s << "\n";  // oops
        outs << *s << "\n";
    }
}

//------------------------------------------------------------------------------
void printShapeNames(std::ostream& outs, const ShapeCollection& toPrint)
{
    /*
    for (const std::unique_ptr<Shape>& s : toPrint) {
        outs << s->name() << "\n";
    }
    */

    std::transform(toPrint.begin(), toPrint.end(),
                   std::ostream_iterator<std::string>(cout, "\n"),
                   [](const std::unique_ptr<Shape>& s) -> std::string {
                       return s->name();
                   });

}
