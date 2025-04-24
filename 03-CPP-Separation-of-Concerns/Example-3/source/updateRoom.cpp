// Thomas Kennedy
// Review Example: Room Update

#include <iostream>
#include <iomanip>
#include <cstdlib>
#include <cmath>
#include <string>
#include <utility>
#include <algorithm>

#include "utilities.h"

#include "Room.h"
#include "House.h"

using namespace std;

/**
 * Build our example house
 */
void buildHouse(House& house);

/**
 * Take a house and change the flooring
 *
 * @param original House to change
 *
 * @return House with the updated flooring
 */
House upgradeFlooring(House original);

/**
 * Compute the area of a room and the cost of
 * flooring for the room
 * <p>
 * Let us Review the use of reference and pointer variables.
 * <p>
 * We will use these when we implement the iterator interface.
 */
int main()
{
    // Construct, build, and print a house
    House house;
    buildHouse(house);

    cout << house;

    // Upgrade the flooring in a
    // second duplicate house
    House duplicateHouse = upgradeFlooring(house);

    cout.setf(ios::boolalpha);
    cout << "\n"
         << "house == duplicateHouse   -> "
         << (house == duplicateHouse)
         << "\n"
         << "&house == &duplicateHouse -> "
         << (&house == &duplicateHouse)
         << "\n"
         << "\n";

    cout << house << "\n";

    cout << duplicateHouse << "\n";

    return 0;
}

//------------------------------------------------------------------------------
void buildHouse(House& house)
{
    // Add the Laundry Room
    house.addRoom(Room("Laundry Room",
                       Room::DimensionSet(8, 4), 1.95, "Laminate"));

    // Add the Kitchen
    house.addRoom(
        Room("Kitchen", Room::DimensionSet(20, 12), 3.87, "Tile")
    );

    // Add the Storage Room
    house.addRoom(Room("Storage Room",
                       Room::DimensionSet(16, 16), 4.39, "Birch Wood"));
}

//------------------------------------------------------------------------------
House upgradeFlooring(House original)
{
    House modified = original;

    // What if we decide to use the same type of
    // Flooring in every room?
    /*
    for (Room& room : modified) {
        room.setFlooring("Stone Bricks", 12.97);
    }
    */
    std::for_each(modified.begin(), modified.end(),
                  [](Room& room) {
                      room.setFlooring("Stone Bricks", 12.97);
                  });

    modified.setName("After Stone Bricks");

    return modified;
}

/*
void doStuff(Room& room) {
    room.setFlooring("Stone Bricks", 12.97);
}*/
