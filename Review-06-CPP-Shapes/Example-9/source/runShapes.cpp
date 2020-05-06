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
#include <functional>

#include "utilities.h"
#include "shapeFactory.h"
// #include "ShapeCollection.h" // No longer needed

/** @file */

using namespace std;
using namespace utilities;

constexpr std::array<string_view, 2> PROGRAM_HEADING = {
    "Objects & Inheritance: 2-D Shapes",
    "Thomas J. Kennedy"
};  ///< Program Title

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
// void printShapes(std::ostream& outs, const ShapeCollection& toPrint);
ostream& operator<<(std::ostream& outs, const ShapeCollection& toPrint);

/**
 * Print shape names for all `Shape`s in a `ShapeCollection` to a
 * specified output stream
 */
void printShapeNames(std::ostream& outs, const ShapeCollection& toPrint);

/**
 * Implementation of a `transform_if` based on `transform` as documented at
 * https://en.cppreference.com/w/cpp/algorithm/transform
 *
 * @tparam InputIt iterator for the source container
 * @tparam OutputIt iterator that allows insertion into an output container
 * @tparam UnaryOp1 boolean function that determines whether include or skip
 *         source data
 * @tparam UnaryOp2 function that transforms data
 *
 * @todo discuss why this is incomplete
 */
template<class InputIt, class OutputIt, class UnaryOp1, class UnaryOp2>
OutputIt transform_if(InputIt itInputStart,
                      InputIt itInputEnd,
                      OutputIt itOutput,
                      UnaryOp1 conditionOperation,
                      UnaryOp2 transformOperation)
{
    while (itInputStart != itInputEnd) {

        if (conditionOperation(*itInputStart)) {
            *itOutput++ = transformOperation(*itInputStart);
        }

        // itInputStart++;
        ++itInputStart;  // Pre-increment avoids a temporary copy.
    }

    return itOutput;
}

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

    cout << generateProjectHeading(PROGRAM_HEADING);

    // Examine the ShapeFactory & list available shapes
    cout << generateHeading<'~', 38>("Available Shapes");

    ShapeFactory::listKnown(cout);
    cout << generateHorizontalLine<'-', 38>();
    cout << right << setw(2)
         << ShapeFactory::numberKnown()
         << " shapes available."
         << "\n";

    cout << "\n";

    // Create 5 "Random" Shapes
    ShapeCollection shapes = readShapes(shapesFile);

    // Print all the shapes
    cout << generateHeading<'~', 38>("Display All Shapes");
    // printShapes(cout, shapes);
    cout << shapes;

    cout << generateHeading<'~', 38>("Display Shape Names");
    // printShapeNames(cout, shapes);  // Should I keep this?
    std::transform(shapes.begin(), shapes.end(),
                   std::ostream_iterator<std::string>(cout, "\n"),
                   [](const unique_ptr<Shape>& shp) -> std::string {
                       return shp->name();
                   });

    //--------------------------------------------------------------------------
    cout << generateHeading<'~', 38>("Display Largest Shape (Area)");
    ShapeCollection::const_iterator it;

    it = std::max_element(shapes.begin(), shapes.end(),
                          [](const auto& lhs, const auto& rhs) {
                              return (lhs)->area() < (rhs)->area();
                          });

    // const unique_ptr<Shape>& data = *it;
    // cout << *data << "\n";
    cout << *(*it) << "\n";

    //--------------------------------------------------------------------------
    cout << generateHeading<'~', 38>("Display Smallest Shape (Perimeter)");
    it = std::min_element(shapes.begin(), shapes.end(),
                          [](const auto& lhs, const auto& rhs) {
                              return (lhs)->perimeter() < (rhs)->perimeter();
                          });
    cout << *(*it) << "\n";

    //--------------------------------------------------------------------------
    cout << generateHeading<'~', 38>("Display Sorted by Name");
    std::sort(shapes.begin(), shapes.end(),
              [](const auto& lhs, const auto& rhs) {
                  return (lhs)->name() < (rhs)->name();
              });

    // printShapes(cout, shapes);
    cout << shapes;

    return 0;
}

//------------------------------------------------------------------------------
ShapeCollection readShapes(std::istream& ins)
{
    ShapeCollection collection;

    std::istream_iterator<Shape*> it(ins);
    std::istream_iterator<Shape*> end;

    // Let us re-enact the Back-to-the-Future I Guitar Scene!
    //
    // std::copy_if would copy raw pointers, not unique pointers.
    //
    // To get our desired result we would need a copy-if folowed by a transform.
    //
    /*
    for_each(it, end,
             [&collection](Shape* s) {
                  if (s != nullptr) {
                      collection.emplace_back(s);
                  }
             });
    */

    transform_if(it, end,
                 std::back_inserter(collection),
                 [](Shape* rawPtr) -> bool {
                     return rawPtr != nullptr;
                 },
                 [](Shape* rawPtr) -> std::unique_ptr<Shape> {
                     return std::unique_ptr<Shape>(rawPtr);
                 });

    return collection;
}

//------------------------------------------------------------------------------
inline
std::ostream& operator<<(std::ostream& outs, const unique_ptr<Shape>& prt)
{
    // prt->display(outs);
    outs << *prt;

    return outs;
}

//------------------------------------------------------------------------------
// void printShapes(std::ostream& outs, const ShapeCollection& toPrint)
ostream& operator<<(std::ostream& outs, const ShapeCollection& toPrint)
{
    /*
    for (const std::unique_ptr<Shape>& s : toPrint) {
        // outs << s << "\n";  // oops
        outs << *s << "\n";
    }
    */

    std::copy(toPrint.begin(), toPrint.end(),
              std::ostream_iterator<unique_ptr<Shape>>(outs, "\n"));

    return outs;
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
                   std::ostream_iterator<std::string>(outs, "\n"),
                   [](const std::unique_ptr<Shape>& s) -> std::string {
                       return s->name();
                   });
}
