#include <string>
#include <iostream>
#include <cstdlib>
#include <cmath>
#include <algorithm>
#include <iterator>
#include <vector>
#include <list>

using namespace std;

int main(int argc, char** argv)
{
    cout << "--------Start Example--------" << '\n';

    list<string> names = {"Thomas", "Jay", "Steve", "Janet", "Ravi"};

    // for (int i = 0; i < names.size(); i++) {
        // cout << "  - " << names[i] << '\n';
    // }


    cout << "--------While Loop Example--------" << '\n';
    list<string>::const_iterator it = names.begin();

    while (it != names.end()) {
        cout << "  - " << *it << '\n';

        ++it;
    }

    cout << "--------For-each Loop Example--------" << '\n';
    for (const string& n : names) {
        cout << "  - " << n << '\n';
    }

    cout << "--------Add a Name--------" << '\n';
    string newName = "Hill";
    names.push_back(newName);

    for (const string& n : names) {
        cout << "  - " << n << '\n';
    }

    cout << "--------Add a Name--------" << '\n';
    string isItThere = "Thomas";

    list<string>::const_iterator sIt = names.begin();

    while (sIt != names.end()) {
        if (*sIt == isItThere) {
            cout << "Found a Match" << "\n";
        }
        ++sIt;
    }


    return 0;
}
