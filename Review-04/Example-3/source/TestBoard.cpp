#include <iostream>
#include <fstream>
#include <string>
#include <cstdlib>
#include <vector>
#include <sstream>
#include <algorithm>
#include <functional>

#include "Board.h"

/**
 * This is the Bodge-Unit-Testing... PseUdO-Framework
 *
 * Bodge - A clumsy or inelegant job, usually a temporary repair;
 * a patch, a repair. (From Wiktionary)
 */

#define bodgeAssert(expression) \
if (!(expression)) { \
    std::cout << " FAILURE: "\
              << __func__ << ":" << __LINE__\
              << " -> (" << #expression << ")\n";\
    return false;\
}
// End Macro

// Unit Test Pseudo-Framework
//-----------------------------------------------------------------------------
using UnitTestFunction = std::function<bool()>;
using UnitTestPair = std::pair<UnitTestFunction, std::string>;


void runTest(const UnitTestFunction& testFunction, std::string description)
{
    std::cout << (testFunction() ? "PASSED" : "FAILED")
              << " -> " << description
              << std::endl;
}

//-----------------------------------------------------------------------------
// Unit Tests - Support Data
//-----------------------------------------------------------------------------

//------------------------------------------------------------------------------
// Unit Tests - Test Functions
//------------------------------------------------------------------------------
/**
 * Helper function for testDisplay.
 *
 * Convert any type with an operator<< defined to a std::string
 */
template<class T>
std::string toStr(const T& thing)
{
    std::ostringstream outs;
    outs << thing;

    return outs.str();
}

bool testBoardButPoorly()
{
    Board board;;

    std::string expectedInitial = "1|2|3\n4|5|6\n7|8|9\n";
    std::string expectedFinal   = "X|X|X\nX|X|X\nX|X|X\n";

    bodgeAssert(toStr(board) == expectedInitial);
    bodgeAssert(!board.isFull());

    for (int i = 0 ; i < 9; i++) {
        board.setCell((i + 1), 'X');
    }

    bodgeAssert(toStr(board) == expectedFinal);
    bodgeAssert(board.isFull());

    return true;
}

// Where are the rest of the tests? This might
// be worth an F... if I am lucky.

//-----------------------------------------------------------------------------
int main(int argc, char** argv)
{
    UnitTestPair tests[] = {
        {testBoardButPoorly, "testBoardButPoorly"}
    };

    for (const UnitTestPair& testPair : tests) {
        runTest(testPair.first, testPair.second);
    }

    return 0;
}
