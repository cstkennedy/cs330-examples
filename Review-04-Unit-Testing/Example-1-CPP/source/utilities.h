#ifndef UTILITIES_H_INCLUDED
#define UTILITIES_H_INCLUDED

#include <string>
#include <sstream>

/**
 * Trim leading and trailing whitespace from a string.
 *
 * @param str string to prune
 *
 * @pre str is nonempty
 */
void trim(std::string& str);

/**
 * Helper function for types where std::to_string is not available.
 *
 * Convert any type with an operator<< defined to a std::string
 */
template<class T>
std::string toStr(const T& thing)
{
    std::ostringstream outs;
    outs << thing;

    return outs.str();
}

#endif
