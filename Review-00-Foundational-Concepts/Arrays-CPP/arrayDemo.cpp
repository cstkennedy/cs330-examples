#include <string>
#include <iostream>
#include <iomanip>
#include <utility>
#include <cmath>


const int MAX_SIZE = 10;  // Maximum allowed array size

const std::string DIVIDER = std::string(20, '-') + "\n";  // A 20 dash divider

/**
 * Statically allocated array demo - keep track of what is actually used
 */
void staticArrayDemo();

/**
 * Dynamically allocated array demo
 */
void dynamicArrayDemo();

/**
 * Print an array using the usual index based for loop.
 *
 * @param toPrint the array to print
 * @param length size of the array (number of elements)
 */
void printArray(const std::string* toPrint, const int length);

/**
 * Print an array using a pointer based loop.
 *
 * @param toPrint the array to print
 * @param length size of the array (number of elements)
 */
void printArrayPtr(const std::string* toPrint, const int length);

//------------------------------------------------------------------------------
int main(int argc, char** argv)
{
    if (argc != 2) {
        std::cerr << "Usage: " << argv[0] << " demoType" << "\n";
        std::cerr << "  demoType: static or dynamic" << "\n";

        return 1;
    }

    const std::string demoType(argv[1]);

    if (demoType == "static") {
        staticArrayDemo();
    }
    else if (demoType == "dynamic") {
        dynamicArrayDemo();
    }
    else {
        std::cerr << '\"' << demoType << '\"'
                  << " is not a valid demo type" << "\n";

        return 2;
    }

    return 0;
}

//------------------------------------------------------------------------------
void staticArrayDemo()
{
    std::string names[MAX_SIZE];
    int used = 0;

    names[used++] = "Thomas";
    names[used++] = "Jay";

    std::cout << DIVIDER;
    for (int i = 0; i < used; i++) {
        std::cout << names[i] << "\n";
    }

    std::cout << DIVIDER;
    printArray(names, used);
    std::cout << DIVIDER;
    printArrayPtr(names, used);
}

//------------------------------------------------------------------------------
void dynamicArrayDemo()
{
    std::string* names = new std::string[2];
    int size = 2;

    names[0] = "Thomas Kennedy";
    names[1] = "Jay Morris";

    // "Resize" - This should be a separate function
    std::string* tempArray = new std::string[size + 1];

    for (int i = 0; i < size; i++) {
        tempArray[i] = names[i];
    }
    delete[] names;
    names = tempArray;
    // End "Resize"

    // Add - should be a separate function (push_back)
    names[2] = "Steve Zeil";
    size = 3;
    // End add

    std::cout << DIVIDER;
    for (int i = 0; i < size; i++) {
        std::cout << names[i] << "\n";
    }

    std::cout << DIVIDER;
    printArray(names, size);
    std::cout << DIVIDER;
    printArrayPtr(names, size);

    delete[] names;
}

//------------------------------------------------------------------------------
void printArray(const std::string* toPrint, const int length)
{
    for (int i = 0; i < length; i++) {
        std::cout << toPrint[i] << "\n";

        // std::cout << *(toPrint + i) << "\n";
        // toPrint[i] == *(toPrint + (i * size(string)))
    }
}

//------------------------------------------------------------------------------
void printArrayPtr(const std::string* toPrint, const int length)
{
    const std::string* begin = toPrint;
    const std::string* end = begin + length;

    const std::string* it = begin;

    while (it != end) {
        std::cout << it << " - " << *it << "\n";

        it++;
    }
}

// Pointer & Iterator "Common" Interface
//
//   - operator++(int v) - post increment - it++
//   - dereference - *it
//   - operator== - it == end
//   - operator!= - it != end
//


