#include <iostream>

using namespace std;

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

    for (int next_even = lower_bound; next_even <= upper_bound; next_even++) {
        cout << next_even << "\n";
    }

    return 0;
}
