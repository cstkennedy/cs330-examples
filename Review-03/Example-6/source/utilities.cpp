#include "utilities.h"

//------------------------------------------------------------------------------
bool promptForYesNo(std::string msg) {
    // Prompt the user for Y/N
    char yn_response = toupper(promptForValue<char>(msg));

    return (yn_response == 'Y');
}
