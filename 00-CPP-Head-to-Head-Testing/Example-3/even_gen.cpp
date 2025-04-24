#include <iostream>

using namespace std;

/**
 * Output all even numbers in a given range.
 *
 * @param lower lower integer bound (a)
 * @param upper upper integer bound (b)
 */
void output_even_integers(const int lower, const int upper)
{
    const int first_even = (lower % 2 == 0 ? lower : lower + 1);

    for (int next_even = first_even; next_even <= upper; next_even += 2) {
        cout << next_even << "\n";
    }
}

int main(int argc, char** argv)
{
    // Check and parse command line args
    if (argc < 3) {
       cout << " Usage: ./even_gen [lower_bound] [upper_bound]" << "\n";
       return 1;
    }

    // Assume all args are well formed (i.e., can be parsed as integers).
    int lower_bound = atoi(argv[1]);
    int upper_bound = atoi(argv[2]);

    // The core even output logic
    cout << "Range [" << lower_bound << ", " << upper_bound << "]" << "\n";
    cout << "\n";

    output_even_integers(lower_bound, upper_bound);

    return 0;
}
