// Thomas Kennedy
// Review Example: Room Update

#include <iostream>
#include <iomanip>
#include <cstdlib>
#include <cmath>
#include <string>
#include <utility>
#include <algorithm>
#include <functional>
#include <numeric>
#include <iterator>

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
 * Compute the discounted flooring price for a single Room.
 *
 * @param r room to examine
 */
inline
double discountFlooring(const Room& r)
{
    return 0.90 * r.flooringCost();
}

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

    cout << house          << "\n";
    cout << duplicateHouse << "\n";

    // Get all the flooring costs with a 10% discount
    vector<double> costs(duplicateHouse.size());
    std::transform(duplicateHouse.begin(), duplicateHouse.end(), costs.begin(),
                   discountFlooring);

    /*for (const Room& rm : duplicateHouse) {
        double cost = 0.9 * rm.flooringCost();
        costs.push_back(cost);
    }*/

    // Print the discounted prices
    /*
    for (auto c : costs) {
        std::cout << c << "\n";
    }*/

    // std::ostream_iterator<double> outIt(std::cout, "\n");
    std::copy(costs.begin(), costs.end(),
              std::ostream_iterator<double>(std::cout, "\n"));

    // Print the sum, min, max -> D.R.Y!
    cout << "Total: "
         << std::accumulate(costs.begin(), costs.end(), 0.0, std::plus<double>())
         << "\n";
    cout << "Min: "
         << *std::min_element(costs.begin(), costs.end())
         << "\n"
         << "Max: "
         << *std::max_element(costs.begin(), costs.end())
         << "\n";

    // I would probably use minmax_element and auto
    /*
    std::pair<std::vector<double>::const_iterator,
              std::vector<double>::const_iterator> extremes = std::minmax_element(costs.begin(), costs.end());
    */
    auto extremes = std::minmax_element(costs.begin(), costs.end());

    cout << "Min: " << *(extremes.first)  << "\n"
         << "Max: " << *(extremes.second) << "\n";

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

    std::for_each(modified.begin(), modified.end(),
                  [](Room& room) {
                      room.setFlooring("Stone Bricks", 12.97);
                  });

    modified.setName("After Stone Bricks");

    return modified;
}
