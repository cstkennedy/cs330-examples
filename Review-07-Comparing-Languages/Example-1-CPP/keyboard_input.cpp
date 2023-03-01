#include <iostream>
#include <iomanip>
#include <string>
#include <limits>

using std::cout;
using std::cin;
using std::setw;
using std::streamsize;
using std::numeric_limits;

constexpr size_t BUFFER_IGNORE_LIMIT = numeric_limits<streamsize>::max();

/**
 * A basic keyboard Input Stream example.
 */
int main(const int argc, const char* const* argv)
{
    int     inputInt             = 0;
    double  inputDouble          = 0;
    char    inputChar            = '\0';
    bool    inputBoolean         = false;

    std::string  inputStringNoSpaces;
    std::string  inputStringWholeLine;

    cout << "Enter an Integer: ";
    cin >> inputInt;

    cout << "Enter a Double: ";
    cin >> inputDouble;

    cout << "Enter a Character: ";
    cin >> inputChar;

    cout << "Enter a Boolean: ";
    cin >> inputBoolean;

    cout << "Enter a string (no spaces): ";
    cin >> inputStringNoSpaces;

    // Same as the C++ getline
    // Strip the trailing newline from the previous read
    cin.ignore(BUFFER_IGNORE_LIMIT, '\n');

    cout << "Enter a string (with spaces): ";
    getline(cin, inputStringWholeLine);

    cout.precision(2);
    cout.setf(std::ios::fixed | std::ios::boolalpha);

    cout << '\n';
    cout << "You Entered:" << '\n';
    cout << "  Item " << setw(2) << 1 << ": " << inputInt << '\n';
    cout << "  Item " << setw(2) << 2 << ": " << inputDouble << '\n';
    cout << "  Item " << setw(2) << 3 << ": " << inputChar << '\n';
    cout << "  Item " << setw(2) << 4 << ": " << inputBoolean << '\n';
    cout << "  Item " << setw(2) << 5 << ": " << inputStringNoSpaces << '\n';
    cout << "  Item " << setw(2) << 6 << ": " << inputStringWholeLine << '\n';

    return 0;
}
