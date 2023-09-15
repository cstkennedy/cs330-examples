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
#include <sstream>

#include "utilities.h"

#include "Room.h"
#include "House.h"

using namespace std;

const std::string ROOM_DATA = R"(Laundry Room; 8 4 1.95 Laminate
Kitchen; 20 12 3.87 Tile
Storage Room; 16 16 4.39 Birch Wood)";

/**
 * Build our example house
 */
void buildHouse(std::istream& ins, House& house);

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

    std::istringstream fakeInputFile(ROOM_DATA);
    buildHouse(fakeInputFile, house);

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
inline
std::istream& operator>>(std::istream &ins, Room& rd)
{
    std::string name;
    double      l, h;
    double      cost;
    std::string flooring;

    ins >> ws;
    getline(ins, name, ';');
    ins >> l >> h >> cost;

    ins >> ws;
    getline(ins, flooring);
    ins >> ws;

    rd = Room(name, Room::DimensionSet(l, h), cost, flooring);

    return ins;
}

//------------------------------------------------------------------------------
void buildHouse(std::istream& ins, House& house)
{
    /*std::string name;
    double      l, h;
    double      cost;
    std::string flooring;

    ins >> ws;
    while(getline(ins, name, ';')) {
        ins >> l >> h >> cost;

        ins >> ws;
        getline(ins, flooring);
        ins >> ws;

        house.addRoom(Room(name, Room::DimensionSet(l, h), cost, flooring));
    }*/

    /*
    Room aRoom;

    while (ins >> aRoom) {
       house.addRoom(aRoom);
    }*/

    std::istream_iterator<Room> ins_it(ins);
    std::istream_iterator<Room> ins_end;

    while (ins_it != ins_end) {
        house.addRoom(*ins_it);

        ins_it++;
    }
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
