#ifndef ITEMSTACK_H_INCLUDED
#define ITEMSTACK_H_INCLUDED

#include <iostream>

#include "ItemFactory.h"

/**
 * A Homogeneous--i.e., uniform--stack of Items.
 */
class ItemStack{
    private:
        Item *item;   ///< Item out of which the stack is composed
        int quantity; ///< Number of items in the stack

    public:
        /**
         * Default to an empty stack composed of Air
         */
        ItemStack();

        /**
         * Create a stack of type *item*
         *
         * @param item Item out of which the stack is composed
         */
        ItemStack( const Item *item );

        /**
         * Copy Constructor - Why is this necessary?
         */
        ItemStack( const ItemStack &src );

        /**
         * Destructor
         */
        ~ItemStack();

        /**
         * Retrieve the Item out of which the stack is composed
         */
        Item* getItem() const;

        /**
         * Retrieve the size of the stack
         */
        int size() const;

        /**
         * Increase the size of the stack
         *
         * @param a number of items to add
         * @pre a > 0 AND permitsStacking() == true
         */
        void addItems( int a );

        /**
         * Does that Item contained in this stack permit stacking?
         *
         * This can be less formally phrased, is this a stackable ItemStack?
         */
        bool permitsStacking() const;

        /**
         * Assignment Operator
         */
        bool operator=( const ItemStack &rhs );

        /**
         * Consider two stacks to be the same if
         * they contain the same type of Item
         */
        bool operator==( const ItemStack &rhs ) const;

        /**
         * Order stacks based on Item name
         */
        bool operator<( const ItemStack &rhs ) const;
};

/**
 * Print the ItemStack directly
 */
std::ostream& operator<<( std::ostream &outs, const ItemStack &prt );

/**
 * 
 */
inline Item* ItemStack::getItem() const{
    return this->item;
}

/**
 * 
 */
inline int ItemStack::size() const{
    return this->quantity;
}

/**
 *
 */
inline bool ItemStack::permitsStacking() const{
    return item->isStackable();
}

/**
 *
 */
inline bool ItemStack::operator==( const ItemStack &rhs ) const{
    return *(this->item) == *(rhs.item);
}

/**
 * 
 */
inline bool ItemStack::operator<( const ItemStack &rhs ) const{
    return *(this->item) < *(rhs.item);
}

#endif