#include "House.h"

/**
 *
 */
House::House()
    :name("House"),
     rooms()
{
}

/**
 *
 */
House::House(std::string name)
    :name(name)
{
    //this->name = name;
}

/**
 *
 */
void House::addRoom(Room toAdd)
{
    rooms.push_back(toAdd);
}

/**
 *
 */
size_t House::size() const {
    return rooms.size();
}

/**
 *
 */
House::iterator House::begin()
{
    return rooms.begin();
}

/**
 *
 */
House::const_iterator House::begin() const
{
    return rooms.begin();
}

/**
 *
 */
House::iterator House::end()
{
    return rooms.end();
}

/**
 *
 */
House::const_iterator House::end() const
{
    return rooms.end();
}

/**
 *
 */
void House::display(std::ostream& outs) const
{
    outs << "--------" << this->name << "--------" << "\n";

    // What type of iterator am I using--i.e.,
    // iterator or const_iterator?

    // Print the rooms
    for (const Room& prtRoom : *this) {
        outs << prtRoom;
    }

    // Compute and print the total
    double                avg   = 0;
    double                total = 0;

    for (const Room& room : *this) {
        total += room.flooringCost();
    }

    avg = total / this->size();

    outs << "\n";
    outs << "------------------------------";
    outs << "\n";

    outs << "Total Cost   : $ " << total << "\n";
    outs << "Avg Room Cost: $ " << avg   << "\n";
}