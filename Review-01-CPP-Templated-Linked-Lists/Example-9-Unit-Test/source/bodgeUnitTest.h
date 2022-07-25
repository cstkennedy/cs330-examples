#ifndef BODGE_UNIT_TEST_H_INCLUDED
#define BODGE_UNIT_TEST_H_INCLUDED

#include <cstdlib>
#include <functional>
#include <iostream>
#include <string>

#include "bodgeUnitTest.h"

/**
 * This is the Bodge-Unit-Testing... PseUdO-Framework
 *
 * Bodge - A clumsy or inelegant job, usually a temporary repair;
 * a patch, a repair. (From Wiktionary)
 */

#define bodgeAssert(expression) \
if (!(expression)) { \
    std::cout << "FAILURE: "\
              << __func__ << ":" << __LINE__\
              << " -> (" << #expression << ")\n";\
    return false;\
}
// End Macro

// Unit Aliases
using UnitTestFunction = std::function<bool()>;
using UnitTestPair = std::pair<UnitTestFunction, std::string>;

/**
 * Run a single unit test function and output PASSED of FAILED based on the
 * result.
 *
 * @TODO I could (and should) probably turn this into a macro.
 */
inline
void runTest(const UnitTestFunction& testFunction, std::string description)
{
    std::cout << "  " << (testFunction() ? "PASSED" : "FAILED")
              << " -> " << description
              << std::endl;
}

#endif
