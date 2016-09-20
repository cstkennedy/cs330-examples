#ifndef ITEMSTACK_H_INCLUDED
#define ITEMSTACK_H_INCLUDED

#include <iostream>

#include "Item.h"

/**
 * A Homogeneous--i.e., uniform--stack of Items.
 */
class ItemStack{
    private:
        Item item;    ///< Item out of which the stack is composed
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
         * @param s size of the stack
         *
         * @pre (s > 0)
         */
        ItemStack( const Item &item, int s );

        /**
         * Retrieve the Item out of which the stack is composed
         */
        Item getItem() const;

        /**
         * Retrieve the size of the stack
         */
        int size() const;

        /**
         * Increase the size of the stack
         *
         * @param a number of items to add
         * @pre a > 0
         */
        void addItems( int a );

        /**
         * Consider two stacks to be the same if
         * they contain the same type of Item
         */
        bool operator==( const ItemStack &rhs ) const;

        /**
         * Order stacks based on Item id
         */
        bool operator<( const ItemStack &rhs ) const;
};

/**
 * Print the ItemStack directly
 */
std::ostream& operator<<( std::ostream &outs, const ItemStack &prt);

/**
 * 
 */
inline Item ItemStack::getItem() const{
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
inline void ItemStack::addItems( int a ){
    this->quantity += a;
}

/**
 *
 */
inline bool ItemStack::operator==( const ItemStack &rhs ) const{
    return this->item == rhs.item;
}

/**
 * 
 */
inline bool ItemStack::operator<( const ItemStack &rhs ) const{
    return this->item < rhs.item;
}

#endif