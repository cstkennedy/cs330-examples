#include <iostream>
#include <fstream>
#include <string>
#include <cstdlib>
#include <vector>
#include <sstream>
#include <algorithm>
#include <functional>

#include "Player.h"


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
//------------------------------------------------------------------------------
using UnitTestFunction = std::function<bool()>;
using UnitTestPair = std::pair<UnitTestFunction, std::string>;

void runTest(const UnitTestFunction& testFunction, std::string description)
{
    std::cout << (testFunction() ? "PASSED" : "FAILED")
              << " -> " << description
              << std::endl;
}

//------------------------------------------------------------------------------
// Unit Tests - Support Data
//------------------------------------------------------------------------------

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

bool testPlayerDefaultConstructor()
{
    Player tom("Tom");

    bodgeAssert("Tom" == toStr(tom));

    return true;
}

// Where are the rest of the tests? This might
// be worth an F... if I am lucky.

//------------------------------------------------------------------------------
int main(int argc, char** argv)
{
    UnitTestPair tests[] = {
        {testPlayerDefaultConstructor, "testPlayerDefaultConstructor"}
    };

    for (const UnitTestPair& testPair : tests) {
        runTest(testPair.first, testPair.second);
    }

    return 0;
}
