#include <cstdlib>
#include <cmath>
#include <iostream>
#include <iomanip>
#include <bitset>
#include <array>
#include <utility>
#include <algorithm>

using namespace std;

const std::string D_LINE(48, '-');  // A 48-dash dividing line
const size_t NUM_POWERS = 32;

/**
 * Main Function - Basics of Pointers & References
 */
int main([[maybe_unused]] const int argc, [[maybe_unused]] const char* const* argv)
{
    std::array<int, NUM_POWERS> powersOfTwo = {0};

    for (size_t i = 0; i < powersOfTwo.size(); i++) {
        powersOfTwo[i] = (1 << i);
    }

    // Output Loop Options
    /*
    for (int i = 0; i < powersOfTwo.size(); i++) {
        cout << powersOfTwo[i] << "\n";
    }

    for (int* it = powersOfTwo.begin(); it < powersOfTwo.end(); it++) {
        cout << *it << "\n";
    }

    for (std::array<int, NUM_POWERS>::iterator it = powersOfTwo.begin(); it < powersOfTwo.end(); it++) {
    for (auto it = powersOfTwo.begin(); it < powersOfTwo.end(); it++) {
        cout << it << " -> " << *it << "\n";
    }
    */

    cout << D_LINE << "\n";

    // for (const auto& v : powersOfTwo) {
    for (const int& v : powersOfTwo) {
        cout << v << "\n";
    }

    cout << "\n";
    cout << D_LINE << "\n";
    cout << "\n";

    for (size_t i = 0; i < NUM_POWERS; i++) {
        cout << right << setw(2)  << i << ": "
             << right << setw(11) << powersOfTwo[i]
             << right << setw(33) << std::bitset<32>(powersOfTwo[i])
             << "\n";
    }

    cout << "\n";
    cout << D_LINE << "\n";
    cout << "\n";

    for (size_t i = 0; i < NUM_POWERS; i++) {
        cout << right << setw(2)  << i << ": "
             << right << setw(11) << powersOfTwo[i]
             << right << setw(33) << &powersOfTwo[i]
             << "\n";
    }

    int sizeInBytes = 0;
    int sizeInBits  = 0;

    sizeInBytes = sizeof(int);
    sizeInBits  = sizeInBytes << 3;

    cout << "\n";
    cout << "sizeof(int) : "
         << sizeInBytes << " (bytes) / "
         << sizeInBits  << " (bits)" << "\n";

    sizeInBytes = sizeof(int&);
    sizeInBits  = sizeInBytes << 3;

    cout << "sizeof(int&): "
         << sizeInBytes << " (bytes) / "
         << sizeInBits  << " (bits)" << "\n";

    sizeInBytes = sizeof(int*);
    sizeInBits  = sizeInBytes << 3;

    cout << "sizeof(int*): "
         << sizeInBytes << " (bytes) / "
         << sizeInBits  << " (bits)" << "\n";

    sizeInBytes = sizeof(powersOfTwo);
    sizeInBits  = sizeInBytes << 3;

    // Set formatting flags
    cout.setf(ios::right);

    cout << "\n";
    cout << "sizeof(powersOfTwo): "
         << setw(4) << sizeInBytes << " (bytes) / "
         << setw(4) << sizeInBits  << " (bits)"
         << "\n";

    sizeInBytes = (sizeof(int) * NUM_POWERS);
    sizeInBits  = sizeInBytes << 3;

    cout << "data size:           "
         << setw(4) << sizeInBytes << " (bytes) / "
         << setw(4) << sizeInBits  << " (bits)"
         << "\n";

    return 0;
}
