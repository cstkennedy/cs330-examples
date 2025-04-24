#include <string>
#include <iostream>
#include <iomanip>
#include <utility>
#include <cmath>


/**
 * Print a value multiple times.
 *
 * @tparam T type to print multiple times
 *
 * @param value value to print
 * @param count number of times to print
 */
template<typename T>
void printMultipleTimes(const T& value, const int count)
{
    for (int i = 0; i < count - 1; i++) {
        std::cout << value << ' ';
    }
    std::cout << value;
}


int main(int argc, char** argv)
{
    printMultipleTimes(7, 3);
    std::cout << "\n";
    printMultipleTimes(5.5, 2);
    std::cout << "\n";
    printMultipleTimes(5.5f, 2);
    std::cout << "\n";
    printMultipleTimes("Hello", 4);
    std::cout << "\n";
    printMultipleTimes('?', 20);
    std::cout << "\n";

    return 0;
}

