#include <iostream>
#include <iomanip>
#include <ctime>
#include <cstdlib>
#include <string>

#include "utilities.h"

#include "LinkedList.h"

/** @file */

using namespace std;
using namespace utilities;

const string PROGRAM_HEADING[] = {
    "Linked List Review",
    "Thomas J. Kennedy"
}; ///< Program Title

const int HEADING_LINES = 2;  ///< Number of lines in Program Heading

const int MIN = -10;  ///< Lower bound for number generation
const int MAX = 10;   ///< Upper bound for number generation

/**
 * Generate a random integer in the range min, max.
 * Default to MIN and MAX
 */
int randomInt(int min = MIN, int max = MAX);

/**
 * Generate a Linked List of random integers
 *
 * @param n number of integers to generate
 */
LinkedList<int> generateList(long n);

/**
 * Linked List Example 2
 */
int main(int argc, char** argv)
{
    int         seed        = 0;  // Seed for random number generation
    long        toGenerate  = 0;  // Number of nodes to generate

    // If a seed was passed from the command line,
    // parse it. Otherwise default to ctime
    if (argc >= 2) {
        seed = atoi(argv[1]);
    }
    else {
        seed = time(NULL);
    }

    // If a node  count was passed from the command line,
    // parse it. Otherwise default to 10
    if (argc >= 3) {
        toGenerate = atoi(argv[2]);
    }
    else {
        toGenerate = 10;
    }

    srand(seed);

    // Print the program heading
    // printProjectHeading(cout, PROGRAM_HEADING, HEADING_LINES);

    // Create a Linked List
    LinkedList<int> randomInts = generateList(toGenerate);

    /*for (int& i : randomInts) {
        std::cout << i << "\n";
    }*/


    return 0;
}

//------------------------------------------------------------------------------
inline
int randomInt(int min, int max)
{
    // Generate a number in the range 0-1
    double x = (1.0 * rand() / RAND_MAX);

    // Compute scaling factor
    int s = max - min + 1;

    // Multiply x by the scaling factor and shift by adding min
    return (x * s) + min;
}

//------------------------------------------------------------------------------
LinkedList<int> generateList(long n)
{
    LinkedList<int> ll;

    // Generate a Linked List (LL) of 10 Nodes
    for (long i = 0; i < n; i++) {
        ll.push_back(randomInt());
    }

    return ll;
}

