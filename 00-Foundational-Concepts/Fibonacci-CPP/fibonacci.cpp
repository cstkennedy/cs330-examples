// Thomas Kennedy
// CS 330
// Review Example: Fibonacci Sequence

#include <iostream>
#include <iomanip>

using namespace std;

/**
 * Note what happens if we do not check the
 * index entered by the user
 *
 * De-Morgan's Law
 * !(index >= 3 && index <= 20)
 * !(index >= 3) || !(index <= 20)
 * (index < 3 || index > 20)
 */
bool __validate_args(const int index)
{
    return (index < 3 || index > 20);
}

/**
 * Generate the Fibonacci Sequence to the n-th number.
 * 1 1 2 3 5 8 13 21 34...
 * <p>
 * The user must enter a number no smaller than 3 and
 * no greater than 20
 */
int main([[maybe_unused]] const int argc, [[maybe_unused]] const char* const* argv)
{
    //--------------------------------------------------------------------------
    // Prompt for sequence length and validate
    //--------------------------------------------------------------------------
    int index = 3;  // Desired length of sequence

    cout << "Generate how many numbers? ";
    cin >> index;
    cout << "\n";

    if (__validate_args(index)) {
        cout << index << " is not between 3 and 20" << "\n";
        return 1;
    }

    //--------------------------------------------------------------------------
    // Compute and output the Fibonaccci Sequence to the index-th term
    //--------------------------------------------------------------------------
    int fm2 = 1;  // n-2 (previous previous) fibonacci number
    int fm1 = 1;  // n-1 (previous) fibonacci number

    // Initial output
    cout << right << " 1: " << setw(10) << fm2 << "\n"
                  << " 2: " << setw(10) << fm1 << "\n";

    // The first 2 numbers were already output
    for (int i = 3; i <= index; i++) {
        const int f = fm1 + fm2;
        fm2 = fm1;
        fm1 = f;

        cout << right << setw(2) << i << ": "
             << setw(10) << f << "\n";
    }

    return 0;
}

