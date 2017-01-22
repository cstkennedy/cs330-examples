#ifndef HOUSE_H_INCLUDED
#define HOUSE_H_INCLUDED

#include <iostream>
#include <string>
#include <list>

#include <cassert>

#include "Room.h"

/**
 * A House is composed of zero or more Room objects.
 * <p>
 * This class serves as our demonstration of the STL
 * iterator interface.
 */
class House{
    public:
        struct Node{
            Room  data;
            Node* next;

            /**
             * Node Constructor
             */
            Node(Room d);
        };

        /**
         * A standard C++ STL style iterator.
         * <p>
         * Recall the rules on Class naming and the STL.
         * <p>
         * Note I have relaxed my rule regarding definitions
         * within class declarations.
         * <p>
         * These are rudimentary iterators. There are a number
         * of additions needed before we can require this complete--e.g.,
         * operator-> and iterator traits. The latter is beyond the scope
         * of this course.
         */
        struct iterator{
            private:
                Node* pseudoPointer;

            public:
                iterator()
                    :pseudoPointer(nullptr)
                {
                }

                iterator(Node* node)
                    :pseudoPointer(node)
                {                    
                }

                Room& operator*() const
                {
                    assert(pseudoPointer != nullptr);
                    return pseudoPointer->data;
                }

                iterator operator++(int v)
                {
                    iterator temp(this->pseudoPointer);
                    
                    this->pseudoPointer = pseudoPointer->next;

                    return temp;
                }

                iterator operator++()
                {
                    this->pseudoPointer = pseudoPointer->next;

                    return *this;
                }

                bool operator== (const iterator &rhs) const
                {
                    return this->pseudoPointer == rhs.pseudoPointer;
                }

                bool operator!= (const iterator &rhs) const
                {
                    return this->pseudoPointer != rhs.pseudoPointer;
                }
        };

        /**
         * A standard C++ STL style const_iterator.
         * <p>
         * Recall the rules on Class naming and the STL.
         */
        struct const_iterator{
            private:
                const Node* pseudoPointer;

            public:
                const_iterator()
                    :pseudoPointer(nullptr)
                {
                }

                const_iterator(const Node* node)
                    :pseudoPointer(node)
                {                    
                }

                const Room& operator*()
                {
                    assert(pseudoPointer != nullptr);
                    return pseudoPointer->data;
                }

                const_iterator operator++(int v)
                {
                    const_iterator ctemp(this->pseudoPointer);
                    
                    this->pseudoPointer = pseudoPointer->next;

                    return ctemp;
                }

                const_iterator operator++()
                {
                    this->pseudoPointer = pseudoPointer->next;

                    return *this;
                }

                bool operator== (const const_iterator &rhs) const
                {
                    return this->pseudoPointer == rhs.pseudoPointer;
                }

                bool operator!= (const const_iterator &rhs) const
                {
                    return this->pseudoPointer != rhs.pseudoPointer;
                }
        };

    private:
        /**
         * Name of the house--e.g.,
         * Minecraft Beach House
         */
        std::string      name;

        /**
         * Container of Rooms
         * <p>
         * This is a pointer to the head (first)
         * Node
         */
        Node*            head;

        /**
         * Container of Rooms
         * <p>
         * This is a pointer to the tail (last)
         * Node
         */
        Node*            tail;

        /**
         * Current size of the house--i.e.,
         * current (actual) number of rooms
         */
        int              currentSize;
        

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
};

/**
 * 
 */
inline
void House::setName(std::string newName)
{
    this->name = newName;
}

/**
 * 
 */
inline
std::string House::getName() const
{
    return (*this).name;
}

/**
 *
 */
inline
bool House::operator==(const House &rhs) const
{
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