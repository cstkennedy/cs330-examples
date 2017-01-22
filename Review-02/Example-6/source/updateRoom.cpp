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
#include "House.h"

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
    House house;

    // Add the Laundry Room
    house.addRoom(
        Room("Laundry Room", Room::DimensionSet(8, 4), 1.95, "Laminate")
    );
    
    // Add the Kitchen
    house.addRoom(
        Room("Kitchen", Room::DimensionSet(20,12), 3.87, "Tile")
    );
    
    // Add the Storage Room
    house.addRoom(
        Room(
            "Storage Room", 
            Room::DimensionSet(16, 16), 
            4.39, 
            "Birch Wood"
        )
    );

    cout << house;

    // What if we decide to use the same type of 
    // Flooring in every room?
    for (Room& room : house) {
        room.setFlooring("Stone Bricks", 12.97);
    }

    cout << "\n";

    house.setName("After Stone Bricks");

    cout << house;

    return 0;   
}
