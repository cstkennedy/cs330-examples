#ifndef AGGREGATE_H_INCLUDED
#define AGGREGATE_H_INCLUDED

#include <iostream>
#include <string>

/**
 * This module contains general purpose functions and constants that can reasonably be 
 * used in any program.
 */

/**
 * Print a blank line - we will discuss the meaning of *inline*
 * in a future Review Session
 */
inline
void println( std::ostream& outs=std::cout )
{
    outs << "\n";
}

/**
 * Prompt the user for a floating point, *double*, value
 *
 * @param msg message to display as a user-prompt
 * 
 * @return double value entered by the user
 */
double promptForDouble( std::string msg );

/**
 * Prompt the user for a Yes or No response
 *
 * @param msg message to display as a user-prompt
 * 
 * @return bool true if the user indicated Yes 
 *     and false otherwise
 */
bool promptForYesNo( std::string msg );

#endif