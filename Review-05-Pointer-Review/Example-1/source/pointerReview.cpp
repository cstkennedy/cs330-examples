#include <cstdlib>
#include <cmath>
#include <iostream>
#include <iomanip>

using namespace std;

/**
 * Array of table widths
 */
const int COL_WIDTHS[4] = {12, 18, 16, 16};

/**
 * Main Function - Basics of Pointers & References
 */
int main(int argc, char** argv)
{
    // Declarations & Definitions 
    int        x               = 9001;
    int*       xPointer        = nullptr;
    int&       xReference      = x;

    const int* xConstPointer   = nullptr;
    const int& xConstReference = x;

    xPointer      = &x;
    xConstPointer = &x;

    // Total Table Width
    int tableWidth = 0;

    // Range based for loop
    for (const int& i : COL_WIDTHS) {
        tableWidth += i;
    }

    // Print Heading
    cout << left  << setw(COL_WIDTHS[0]) << "Type" 
         << left  << setw(COL_WIDTHS[1]) << "Variable"
         << right << setw(COL_WIDTHS[2]) << "Address "
         << right << setw(COL_WIDTHS[3]) << "Value"
         << "\n";

    // Divider
    cout << std::string(tableWidth, '-')
         << "\n";

    // Print Content
    cout << left  << setw(COL_WIDTHS[0]) << "int" 
         << left  << setw(COL_WIDTHS[1]) << "x"
         << right << setw(COL_WIDTHS[2]) << &x
         << right << setw(COL_WIDTHS[3]) << x
         << "\n";

    cout << left  << setw(COL_WIDTHS[0]) << "int*" 
         << left  << setw(COL_WIDTHS[1]) << "xPointer"
         << right << setw(COL_WIDTHS[2]) << &xPointer
         << right << setw(COL_WIDTHS[3]) << xPointer
         << "\n";

    cout << left  << setw(COL_WIDTHS[0]) << "int&" 
         << left  << setw(COL_WIDTHS[1]) << "xReference"
         << right << setw(COL_WIDTHS[2]) << &xReference
         << right << setw(COL_WIDTHS[3]) << xReference
         << "\n";

    // Divider
    cout << std::string(tableWidth, '-')
         << "\n";

    cout << left  << setw(COL_WIDTHS[0]) << "const int*" 
         << left  << setw(COL_WIDTHS[1]) << "xConstPointer"
         << right << setw(COL_WIDTHS[2]) << &xConstPointer
         << right << setw(COL_WIDTHS[3]) << xConstPointer
         << "\n";

    cout << left  << setw(COL_WIDTHS[0]) << "const int&" 
         << left  << setw(COL_WIDTHS[1]) << "xConstReference"
         << right << setw(COL_WIDTHS[2]) << &xConstReference
         << right << setw(COL_WIDTHS[3]) << xConstReference
         << "\n";

    return 0;
}