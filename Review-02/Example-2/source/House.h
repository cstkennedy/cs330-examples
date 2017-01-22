#ifndef HOUSE_H_INCLUDED
#define HOUSE_H_INCLUDED

#include <iostream>
#include <string>
#include <vector>

#include "Room.h"

/**
 * A House is composed of zero or more Room objects.
 * <p>
 * This class serves as our demonstration of the STL
 * iterator interface.
 */
class House{
    public:

    private:
        /**
         * Name of the house--e.g.,
         * Minecraft Beach House
         */
        std::string name;

        /**
         * Container of Rooms
         */
        std::vector<Room> rooms;
    
    public:
        /**
         * Construct a House with a
         * generic name and no rooms.
         */
        House();

        /**
         * Construct a House with a specified name
         */
        House(std::string name);

        /**
         * Add a Room
         *
         * @param toAdd new Room object
         */
        void addRoom(Room toAdd);

        /**
         * Print the house
         */
        void display(std::ostream& outs) const;
};

/**
 * House Stream Insertion (Output) Operator
 * 
 * This is often written as a wrapper for a 
 * display or print function.
 * <p>
 * This operator can *NOT* be implemented as a member function.
 */
inline
std::ostream& operator<<(std::ostream &outs, const House &prt)
{
    prt.display(outs);

    return outs;
}


#endif