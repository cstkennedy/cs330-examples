#include <iostream>
#include <fstream>
#include <string>
#include <cstdlib>
#include <vector>
#include <sstream>
#include <algorithm>
#include <functional>

// #include "utilities.h"

#include "bodgeUnitTest.h"
#include "LinkedList.h"

//-----------------------------------------------------------------------------
// Inventory Unit Tests - Support Data
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
//  Linked List - Unit Tests - Test Functions
//-----------------------------------------------------------------------------
bool testDefaultConstructor()
{
    LinkedList<int> emptyList;

    bodgeAssert(emptyList.size() == 0);

    return true;
}

//-----------------------------------------------------------------------------
bool testPushBack()
{
    LinkedList<int> list;

    list.push_back(0);
    list.push_back(-7);
    list.push_back(1337);

    bodgeAssert(list.size() == 3);

    // Test Pre-Increment
    LinkedList<int>::iterator it = list.begin();
    bodgeAssert(*it == 0);
    ++it;
    bodgeAssert(*it == -7);
    ++it;
    bodgeAssert(*it == 1337);
    ++it;
    bodgeAssert(it == list.end());

    // Test Post-Increment
    it = list.begin();
    bodgeAssert(*it == 0);
    it++;
    bodgeAssert(*(it++) == -7);
    bodgeAssert(*it == 1337);
    ++it;
    bodgeAssert(it == list.end());

    // const LinkedList<int>& constList = list;
    // LinkedList<int>::const_iterator const_it = constList.begin();

    return true;
}

//-----------------------------------------------------------------------------
int main(int argc, char** argv)
{
    UnitTestPair tests[] = {
        {testDefaultConstructor, "testDefaultConstructor"},
        {testPushBack, "testPushBack"},
    };

    std::cout << "Linked List:" << "\n";
    for (const UnitTestPair& testPair : tests) {
        runTest(testPair.first, testPair.second);
    }

    return 0;
}

