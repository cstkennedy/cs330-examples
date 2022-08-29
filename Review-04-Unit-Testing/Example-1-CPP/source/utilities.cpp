#include <string>
#include <cstdlib>

#include "utilities.h"

/**
 * Trim leading and trailing whitespace from a string.
 *
 * @param str string to prune
 *
 * @pre str is nonempty
 */
void trim(std::string& str)
{
    if (str.empty()) {
        return;
    }

    const int first_nonspace = str.find_first_not_of(" \t");
    const int last_non_space = str.find_last_not_of(" \t");

    str = str.substr(first_nonspace, last_non_space + 1);
}

