#include "House.h"

/**
 *
 */
House::House()
    :name("House"),
     rooms()
{
    currentSize = 0;
}

/**
 *
 */
House::House(std::string name)
    :name(name)
{
    currentSize = 0;
}

/**
 *
 */
void House::addRoom(Room toAdd)
{
    //rooms[currentSize] = toAdd;
    //currentSize++;

    rooms[currentSize++] = toAdd;
}

/**
 *
 */
size_t House::size() const {
    return currentSize;
}

/**
 *
 */
House::iterator House::begin()
{
    return &(rooms[0]);
}

/**
 *
 */
House::const_iterator House::begin() const
{
    return &rooms[0];
}

/**
 *
 */
House::iterator House::end()
{
    return &rooms[currentSize];
}

/**
 *
 */
House::const_iterator House::end() const
{
    return &rooms[currentSize];
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