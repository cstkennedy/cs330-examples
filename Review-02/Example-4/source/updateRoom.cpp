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

    Room room("Laundry Room", Room::DimensionSet(8, 4), 1.95, "Laminate");
    Room kitchen("Kitchen", Room::DimensionSet(20,12), 3.87, "Tile");

    House house;

    house.addRoom(room);
    house.addRoom(kitchen);

    cout << house;

    // How do I access all the rooms to compute the average cost?
    // How do I compute the total cost?
    // Let us use a quick scope trick
    {
        House::const_iterator it    = house.begin();
        double                avg   = 0;
        double                total = 0;

        while (it != house.end()) {
            total += it->flooringCost();

            it++;
        }

        avg = total / house.size();

        cout << "\n";
        cout << "------------------------------";
        cout << "\n";

        cout << "Total Cost   : $ " << total << "\n";
        cout << "Avg Room Cost: $ " << avg   << "\n";
    }

    // Let us use a quick scope trick
    {        
        double                avg   = 0;
        double                total = 0;

        for (const Room& room : house) {
            total += room.flooringCost();
        }

        avg = total / house.size();

        cout << "\n";
        cout << "------------------------------";
        cout << "\n";

        cout << "Total Cost   : $ " << total << "\n";
        cout << "Avg Room Cost: $ " << avg   << "\n";
    }

    // What if we decide to use the same type of 
    // Flooring in every room
    for (Room& room : house) {
        room.setFlooring("Stone Bricks", 12.97);
    }

    double                avg   = 0;
    double                total = 0;

    for (const Room& room : house) {
        total += room.flooringCost();
    }

    avg = total / house.size();

    cout << "\n";
    cout << "------------------------------";
    cout << "\n";

    cout << "Total Cost   : $ " << total << "\n";
    cout << "Avg Room Cost: $ " << avg   << "\n";

    return 0;   
}
