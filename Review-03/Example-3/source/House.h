#ifndef HOUSE_H_INCLUDED
#define HOUSE_H_INCLUDED

#include <iostream>
#include <string>
#include <list>
#include <utility>

#include <cassert>

#include "Room.h"
#include "LinkedList.h"

#include <vector>
#include <list>

/**
 * A House is composed of zero or more Room objects.
 * <p>
 * This class serves as our demonstration of the STL
 * iterator interface.
 */
class House {
    private:
        using Collection = LinkedList<Room>;

    public:
        using iterator       = Collection::iterator;
        using const_iterator = Collection::const_iterator;

    private:
        /**
         * Name of the house--e.g.,
         * Minecraft Beach House
         */
        std::string name;

        Collection rooms;

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
         * Update the name
         */
        void setName(std::string newName);

        /**
         * Update the name
         */
        std::string getName() const;

        /**
         * Allow access to the _beginning_ of the
         * house--i.e., Room container--via an
         * iterator.
         */
        iterator begin();

        /**
         * Allow access to the _beginning_ of the
         * house--i.e., Room container--via a
         * const_iterator.
         */
        const_iterator begin() const;

        /**
         * Allow access to the _end_ of the
         * house--i.e., Room container--via an
         * iterator.
         */
        iterator end();

        /**
         * Allow access to the _end_ of the
         * house--i.e., Room container--via a
         * const_iterator.
         */
        const_iterator end() const;

        /**
         * Return the size of the house--i.e.,
         * the number of rooms
         */
        size_t size() const;

        /**
         * Print the house
         */
        void display(std::ostream& outs) const;

        /**
         * Logical Equivalance Operator
         */
        bool operator==(const House &rhs) const;

        /**
         * Swap the contents of two `House`s
         * <p>
         * I am using a friend function here and only here (under protest)
         * <p>
         * [Refer here](http://stackoverflow.com/questions/3279543/what-is-the-copy-and-swap-idiom)
         */
        friend
        void swap(House& lhs, House& rhs);
};

//------------------------------------------------------------------------------
inline
void House::setName(std::string newName)
{
    this->name = newName;
}

//------------------------------------------------------------------------------
inline
std::string House::getName() const
{
    return (*this).name;
}

//------------------------------------------------------------------------------
inline
bool House::operator==(const House &rhs) const
{
    // **The compiler most likely ignored the inline keyword**

    if (this->name != rhs.name) {
        return false;
    }

    const_iterator lhsIt = this->begin();
    const_iterator rhsIt = rhs.begin();

    while (lhsIt != this->end() && rhsIt != rhs.end()) {
        if (*lhsIt != *rhsIt) {
            return false;
        }

        lhsIt++;
        rhsIt++;
    }

    if (lhsIt == this->end() && rhsIt == rhs.end()) {
        return true;
    }

    return false;
}

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
