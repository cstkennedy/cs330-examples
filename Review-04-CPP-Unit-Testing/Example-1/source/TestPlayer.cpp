#include <iostream>
#include <fstream>
#include <string>
#include <cstdlib>
#include <vector>
#include <sstream>
#include <algorithm>
#include <functional>

#include "Player.h"

#include "bodgeUnitTest.h"
#include "utilities.h"


//-----------------------------------------------------------------------------
// Unit Tests - Support Data
//-----------------------------------------------------------------------------


//-----------------------------------------------------------------------------
// Unit Tests - Test Functions
//-----------------------------------------------------------------------------
bool testDefaultConstructor()
{
    Player some_person;

    std::cout << some_person.getName() << '\n';
    // std::cout << some_person.getSymbol() << '\n';
    // std::cout << some_person.isHuman() << '\n';
    // std::cout << some_person.isComputer() << '\n';

    /*
    if (!some_person.isHuman()) {
        std::cout << "Fail: Player is not human" << "\n";
        return false;
    }
    */

    bodgeAssert(some_person.isHuman());

    /*
    if (some_person.isComputer()) {
        std::cout << "Fail: Player is a Computer" << "\n";
        return false;
    }
    */

    bodgeAssert(!some_person.isComputer());

    return true;
}

//------------------------------------------------------------------------------
int main(int argc, char** argv)
{
    UnitTestPair tests[] = {
        {testDefaultConstructor, "testDefaultConstructor"},
    };
    runTests(tests);

    return 0;
}

