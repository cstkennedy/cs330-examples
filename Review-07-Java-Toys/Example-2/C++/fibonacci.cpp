// Thomas Kennedy
// CS 330
// Review Example: Fibonacci Sequence

#include <iostream>
#include <iomanip>

using namespace std;

/**
 * Generate the Fibonacci Sequence to the n-th number.
 * 1 1 2 3 5 8 13 21 34...
 * <p>
 * The user must enter a number no smaller than 3 and
 * no greater than 20
 */
int main(int argc, char** argv)
{
    int index = 3; // Desired length of sequence

    // Fibonaccci
    int   fm2 = 1; // n-2 (previous previous) fibonacci number
    int   fm1 = 1; // n-1 (previous) fibonacci number
    int   f   = 0; // current fibonacci number

    // Prompt the user
    cout << "Generate how many numbers? ";
    cin >> index;

    // Print a blank line
    cout << "\n";

    /*
    Note what happens if we do not check the
    index entered by the user

    De-Morgan's Law
    !(index >= 3 && index <= 20)
    !(index >= 3) || !(index <= 20)
    (index < 3 || index > 20)
    */    
    if(index < 3 || index > 20) {       
        // Error Message
        cout << index << " is not between 3 and 20" << "\n";
        
        // Exit with an error state
        return 1;
    }

    // Initial output
    cout << right << " 1: " << setw(10) << fm2 << "\n"
                  << " 2: " << setw(10) << fm1 << "\n";

    // The first 2 numbers were already output
    for(int i = 3; i <= index; i++) {
        f   = fm1 + fm2;
        fm2 = fm1;
        fm1 = f;

        cout << right << setw(2) << i << ": " 
             << setw(10) << f << "\n";
    }

    return 0;   
}

