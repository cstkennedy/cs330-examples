// CS 330 Fall 2016
// Review Example 1: Linked List Review

#include <iostream>
#include <iomanip>
#include <ctime>
#include <cstdlib>
#include <string>

#include "utilities.h"

#include "node.h"
#include "linkedList.h"

/** @file */

using namespace std;
using namespace utilities;

const string PROGRAM_HEADING[] = {
    "Linked List Review",
    "Thomas J. Kennedy"
}; ///< Program Title 

const int HEADING_LINES = 2; ///< Number of lines in Program Heading

const int MIN = -10; ///< Lower bound for number generation
const int MAX = 10;  ///< Upper bound for number generation

/**
 * Generate a random integer in the range min, max.
 * Default to MIN and MAX
 */
int randomInt(int min=MIN, int max=MAX);

/**
 * Linked List Example 2
 */
int main(int argc, char** argv)
{
    LinkedList* random_ints = nullptr; // List of random integers

    int         seed        = 0; // Seed for random number generation
    int         to_generate;     // Number of nodes to generate    

    // If a seed was passed from the command line,
    // parse it. Otherwise default to ctime
    if( argc >= 2 ) {
        seed = atoi( argv[1] );          
    }
    else {
        seed = time( NULL );
    }

    // If a node  count was passed from the command line,
    // parse it. Otherwise default to 10
    if( argc >= 3 ) {
        to_generate = atoi( argv[2] );   
    }
    else {
        to_generate = 10;
    } 

    srand( seed );    

    // Print the program heading
    printProjectHeading( cout, PROGRAM_HEADING, HEADING_LINES );

    // Create a Linked List
    random_ints = new LinkedList();

    // Generate a Linked List (LL) of 10 Nodes
    for( int i = 0; i < to_generate; i++ ){     
        random_ints->appendNode( randomInt() );
    }

    // Print the LL
    cout << (*random_ints);

    printHorizontalLine( cout, '*', W_WIDTH );

    // Delete the Linked List
    // Which function is called?
    delete random_ints;

    return 0;
}

/**
 *
 */
int randomInt( int min, int max )
{
    // Generate a number in the range 0-1
    double x = (1.0 * rand() / RAND_MAX);

    // Compute scaling factor
    int s = max - min + 1;

    // Multiply x by the scaling factor and shift
    // by adding min
    return ( x * s ) + min; 
}