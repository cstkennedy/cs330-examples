#include <string>
#include <iostream>
#include <cstdlib>
#include <cmath>
#include <algorithm>
#include <iterator>
#include <vector>
#include <list>

using namespace std;


/**
 * Search for a name
 *
 * @tparam Iterator type of iterator (dependent on data structure)
 *
 * @param start where to begin the search
 * @param end where to end the search
 * @param thingToFind name to locate
 *
 * @returns postion of a match or end if no match was found
 */
template<class Iterator>
Iterator findName(Iterator start, Iterator end, const string thingToFind)
{
    Iterator searchIt = start;

    while (searchIt != end) {
        // Look at the current name (*searchIt) and
        // compare to the name to find (thingToFind)
        if (*searchIt == thingToFind) {
            return searchIt;
        }
        ++searchIt;
    }

    return end;
}

using Collection = std::vector<string>;

int main(int argc, char** argv)
{
    cout << "--------Start Example--------" << '\n';

    Collection names = {"Thomas", "Jay", "Steve", "Janet", "Ravi"};

    // Can only be used with vectors
    /*
    for (int i = 0; i < names.size(); i++) {
        cout << "  - " << names[i] << '\n';
    }
    */

    cout << "---------While Loop Example----------" << '\n';

    Collection::const_iterator it = names.begin();

    while (it != names.end()) {
        // Dereference
        // Node: it->data
        // Iterator: *it (* is called the dereference operator)
        cout << "  - " << *it << '\n';

        ++it;
    }

    cout << "--------For-each Loop Example--------" << '\n';
    for (const string& n : names) {
        cout << "  - " << n << '\n';
    }

    //--------------------------------------------------------------------------
    // Add a name - push_back demo
    //--------------------------------------------------------------------------
    cout << "-------------Add a Name--------------" << '\n';
    string newName = "Hill";
    names.push_back(newName);

    for (const string& n : names) {
        cout << "  - " << n << '\n';
    }

    //--------------------------------------------------------------------------
    // Search for a name
    //--------------------------------------------------------------------------
    cout << "----------Search for a Name----------" << '\n';
    string thingToFind = "Thomas";

    // Original non-function search
    /*
    Collection::const_iterator searchIt = names.begin();

    while (searchIt != names.end()) {
        // Look at the current name (*searchIt) and
        // compare to the name to find (thingToFind)
        if (*searchIt == thingToFind) {
            cout << "Found a Match" << "\n";
        }
        ++searchIt;
    }
    */

    const auto foundIt = findName(names.begin(), names.end(), "Andrey");

    if (foundIt == names.end()) {
        cout << "No match was found" << "\n";
    }
    else {
        cout << "Found a Match" << "\n";
    }

    const bool wasFound = std::find(names.begin(), names.end(), "Steve") != names.end();
    if (wasFound) {
        cout << "Found a Match" << "\n";
    }
    else {
        cout << "No match was found" << "\n";
    }

    return 0;
}
