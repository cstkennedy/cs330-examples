#include <cstdlib>
#include <cmath>
#include <iostream>
#include <iomanip>

#include "ColumnHeading.h"

using namespace std;

/**
 * Array of table widths
 */
const int COL_WIDTHS[4] = {12, 18, 16, 16};

/**
 * Column Headings
 */
const ColumnHeading TITLE_ROW[] = {
    ColumnHeading(COL_WIDTHS[0], "Type"    , ColumnHeading::Alignment::LEFT),
    ColumnHeading(COL_WIDTHS[1], "Variable", ColumnHeading::Alignment::LEFT),
    ColumnHeading(COL_WIDTHS[2], "Address" , ColumnHeading::Alignment::RIGHT),
    ColumnHeading(COL_WIDTHS[3], "Value"   , ColumnHeading::Alignment::RIGHT)
};

/**
 * Print a Row while preserving types and addreses.
 */
template<typename T>
void printRow(std::string type, std::string varName, const T& var);

/**
 * Main Function - Basics of Pointers & References
 */
int main(int argc, char** argv)
{
    // Declarations & Defintions 
    int        x               = 9001;
    int*       xPointer        = nullptr;
    int&       xReference      = x;

    const int* xConstPointer   = nullptr;
    const int& xConstReference = x;

    xPointer      = &x;
    xConstPointer = &x;

    // Total Table Width
    int tableWidth = 0;

    // Print Heading
    for (const ColumnHeading& colHeading : TITLE_ROW) {
        cout << colHeading;

        tableWidth += colHeading.width;
    }

    cout << "\n";

    // Divider
    cout << std::string(tableWidth, '-')
         << "\n";

    // Print Content
    printRow("int", "x", x);
    printRow("int*", "xPointer", xPointer);
    printRow("int&", "xReference", xReference);

    // Divider
    cout << std::string(tableWidth, '-')
         << "\n";

    printRow("const int*", "xConstPointer", xConstPointer);
    printRow("const int&", "xConstReference", xConstReference);

    return 0;
}

template<typename T>
void printRow(std::string type, std::string varName, const T& var)
{
    cout << left  << setw(COL_WIDTHS[0]) << type 
         << left  << setw(COL_WIDTHS[1]) << varName
         << right << setw(COL_WIDTHS[2]) << &var
         << right << setw(COL_WIDTHS[3]) << var
         << "\n";
}