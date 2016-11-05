#include <iomanip>
#include <utility>

#include "Inventory.h"

// Allow the compiler to define the remaining 
// comparison operators
using namespace std::rel_ops;

/**
 *
 */
Inventory::Inventory(){
    this->capacity    = 10;
}

/**
 *
 */
Inventory::Inventory( int n ){
    this->capacity = n;
}

/**
 *
 */
bool Inventory::addItems( ItemStack stack ){
    Iterator it = slots.begin();

    // Traverse the list of Items until either the end is
    // reached or a matching item is found
    while( it != slots.end() && (*it != stack) ){
        //std::cerr << ( it->getItem()->getName() ) << " == " 
        //          << ( stack.getItem()->getName() ) << "\n";
        it++;
    }

    // If no match was found and the capacity has not
    // yet been reached, store the Item
    if( it == slots.end() && (slots.size() != capacity) ){
        slots.push_back( stack );

        return true;
    }

    // if a match was found
    if( it != slots.end() ){
        // If the Item is stackable, add it to the ItemStack
        // If the item is not stackable and the Inventory is not full
        // store the Item in a new slot
        if( it->permitsStacking() ){            
            it->addItems( stack.size() );            
            return true;
        }
        else if( slots.size() < capacity ){
            slots.push_back( stack );
            return true;
        }
        return false;
    }
    return false;
}

/**
 *
 */
void Inventory::display( std::ostream &outs ) const{
    // Compute % full rounded to nearest whole number
    int percent_filled = (100.0 * slots.size() / capacity ) + 0.5;

    // Print the usage summary
    outs << " -Used " << std::right << std::setw(3)
         << percent_filled << "\% of " 
         << capacity << " slots" << "\n";

    // Print the Items
    for( ItemStack slot : slots ){
        outs << slot << "\n";
    }
}