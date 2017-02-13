#include <cstdlib>
#include <cmath>
#include <iostream>
#include <iomanip>
#include <bitset>

using namespace std;

/**
 * Array of table widths
 */
const int COL_WIDTHS[] = {12, 18, 16, 16};

/**
 * Main Function - Basics of Pointers & References
 */
int main(int argc, char** argv)
{
    int* powersOfTwo = new int();

    powersOfTwo[0] = 1;
    //*powersOfTwo   = 1;

    // Working with a pointer to a singleton
    cout << "powersOfTwo    = " << powersOfTwo    << "\n"
         << "&powersOfTwo   = " << &powersOfTwo   << "\n"
         << "\n";

    // Dereferencing a pointer & using offsets
    cout << "*powersOfTwo   = " << *powersOfTwo   << "\n";
    cout << "powersOfTwo[0] = " << powersOfTwo[0] << "\n";
  
    cout << "\n";
    cout << "Using a traditional for loop" << "\n";
    for(int i = 0; i < 1; i++) {
        cout << powersOfTwo[i] << "\n";
    }

    delete powersOfTwo;

    cout << std::string(48, '-') << "\n";

    // Working with a pointer to an array
    const int numPowers = 32;

    powersOfTwo = new int[numPowers];

    for (int i = 0; i < numPowers; i++) {
        powersOfTwo[i] = (1 << i);
    }

    for (int* it = powersOfTwo; it < powersOfTwo + numPowers; it++) {
        cout << it << " -> " << *it << "\n";
    }

    cout << "\n";
    cout << std::string(48, '-') << "\n";
    cout << "\n";

    for (int i = 0; i < numPowers; i++) {
        cout << right << setw(2)  << i << ": " 
             << right << setw(11) << powersOfTwo[i] 
             << right << setw(33) << std::bitset<32>(powersOfTwo[i])
             << "\n"; 
    }

    cout << "\n";
    cout << std::string(48, '-') << "\n";
    cout << "\n";

    for (int i = 0; i < numPowers; i++) {
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
    cout << "sizeof(int) : " << sizeInBytes << " (bytes) / "
                             << sizeInBits  << " (bits)" 
         << "\n";

    sizeInBytes = sizeof(int&);
    sizeInBits  = sizeInBytes << 3;

    cout << "sizeof(int&): " << sizeInBytes << " (bytes) / "
                             << sizeInBits  << " (bits)"  
         << "\n";

    sizeInBytes = sizeof(int*);
    sizeInBits  = sizeInBytes << 3;

    cout << "sizeof(int*): " << sizeInBytes << " (bytes) / "
                             << sizeInBits  << " (bits)"  
         << "\n";

    sizeInBytes = sizeof(powersOfTwo);
    sizeInBits  = sizeInBytes << 3;

    // Set formatting flags
    cout.setf(ios::right);

    cout << "\n";
    cout << "sizeof(powersOfTwo): " 
         << setw(4) << sizeInBytes 
         << " (bytes) / "
         << setw(4) << sizeInBits  
         << " (bits)"  
         << "\n";

    sizeInBytes = (sizeof(int) * numPowers);
    sizeInBits  = sizeInBytes << 3;

    cout << "data size:           " 
         << setw(4) << sizeInBytes 
         << " (bytes) / "
         << setw(4) << sizeInBits  
         << " (bits)"  
         << "\n";

    return 0;
}