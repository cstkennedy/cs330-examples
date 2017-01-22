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
    // Why are these lines invalid?
    //Room room("Laundry Room", 8, 4, 1.95, "Laminate");
    //Room kitchen("Kitchen", 20,12, 3.87, "Tile");

    // Why are these lines invalid?
    //Room room("Laundry Room", DimensionSet(8, 4), 1.95, "Laminate");
    //Room kitchen("Kitchen", DimensionSet(20,12), 3.87, "Tile");

    Room  room("Laundry Room", Room::DimensionSet(8, 4), 1.95, "Laminate");
    Room  kitchen("Kitchen", Room::DimensionSet(20,12), 3.87, "Tile");

    House house;

    house.addRoom(room);
    house.addRoom(kitchen);

    cout << house;

    // How do I access all the rooms to compute the average cost?
    // How do I compute the total cost?

    return 0;   
}
