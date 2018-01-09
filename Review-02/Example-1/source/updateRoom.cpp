// Thomas Kennedy
// Review Example: Room Update

#include <iostream>
#include <iomanip>
#include <cstdlib>
#include <cmath>
#include <string>
#include <utility>

#include "utilities.h"

#include "Room.h"

using namespace std;

/**
 * Compute the area of a room and the cost of 
 * flooring for the room
 * <p>
 * Let us Review the use of reference and pointer variables.
 * <p>
 * We will used these when we implement the iterator interface.
 */
int main()
{
    Room room("Laundry Room", 8, 4, 1.95);
    Room kitchen("Kitchen", 20,12, 3.87);

    Room* roomPointer = &room;
    
    room.setFlooring("Tile", 2.50);

    cout << "----------------------------------------------------------------"
         << "\n";

    cout << room;

    cout << "----------------------------------------------------------------"
         << "\n";

    cout << (*roomPointer);

    cout << "----------------------------------------------------------------"
         << "\n";

    if (room == *roomPointer) {
        cout << "room == *roomPointer";
    }
    else {
        cout << "room != *roomPointer";
    }

    cout << "\n"
         << "----------------------------------------------------------------"
         << "\n";

    if (room == kitchen) {
        cout << "room == kitchen";
    }
    else {
        cout << "room != kitchen";
    }

    cout << "\n";

    if (room < kitchen) {
        cout << "room < kitchen";
    }
    else {
        cout << "room >= kitchen";
    }

    // I missed Java, so I invented this function
    println();

    return 0;   
}
