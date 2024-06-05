#include <iostream>
#include <fstream>
#include <string>
#include <cstdlib>
#include <vector>
#include <sstream>
#include <algorithm>
#include <functional>

#include "Board.h"

#include "utilities.h"
#include "bodgeUnitTest.h"

//-----------------------------------------------------------------------------
// Unit Tests - Support Data
//-----------------------------------------------------------------------------

//------------------------------------------------------------------------------
// Unit Tests - Test Functions
//------------------------------------------------------------------------------

bool testBoardButPoorly()
{
    Board board;

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

    runTests(tests);

    return 0;
}
