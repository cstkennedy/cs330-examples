#include "utilities.h"

using namespace std;

/**
 *
 */
double promptForDouble( std::string msg )
{
    double value = 0;

    cout << msg << ": ";
    cin >> value;

    return value;
}

/**
 *
 */
bool promptForYesNo( std::string msg )
{
    // Prompt the user for Y/N
    char yn_response = 'n';

    cout << msg << ": ";
    cin >> yn_response;

    yn_response = toupper( yn_response );

    return (yn_response == 'Y');
}
