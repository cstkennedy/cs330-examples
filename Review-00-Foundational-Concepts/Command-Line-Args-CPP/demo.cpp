#include <iostream>
#include <iomanip>
#include <cstdlib>

using namespace std;


int main(int argc, char** argv)
{
    for (int i = 0; i < argc; ++i) {
        cout << right << setw(3) << i << ": "
             << left << argv[i]
             << "\n";
    }

    return 0;
}
