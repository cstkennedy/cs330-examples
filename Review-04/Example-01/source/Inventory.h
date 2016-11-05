#ifndef INVENTORY_H_INCLUDED
#define INVENTORY_H_INCLUDED

#include <iostream>
#include <list>

#include "ItemStack.h"

/**
 * An Inventory is composed of n slots. Each slot may store only
 * one type of item--specified by *slots*. 
 * <p>
 * Once all slots are filled, no additional Item types may be
 * stored. Individual slots may contain any number of the same 
 * Item--if the Item is stackable.
 * <p>
 * For brevity, the full C++ STL iterator interface is not implemented.
 */
class Inventory{
    private:        
        std::list<ItemStack> slots;  ///< Individual item slots--each ItemStack represents one slot
        int capacity;                ///< Maximum inventory size

        typedef std::list<ItemStack>::iterator Iterator; ///< Inventory Slot Iterator
    public:
        /**
         * Default to 10 slots
         */
        Inventory();

        /**
         * Create an inventory with n slots
         *
         * @pre n > 0
         */
        Inventory( int n );

        /**
         * Add one or more items to the inventory list
         *
         * @return true if *stack* was added and false otherwise
         */
        bool addItems( ItemStack stack );

        /**
         * Print a Summary of the Inventory and all Items contained within
         */
        void display( std::ostream &outs ) const;
};

/**
 * Print the Inventory through use of the display member function
 */
inline std::ostream& operator<<( std::ostream &outs, const Inventory &prt ){
    prt.display( outs );
    return outs;
}

#endif