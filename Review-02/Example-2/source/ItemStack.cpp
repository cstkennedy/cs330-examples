#include <iomanip>

#include "ItemStack.h"

/**
 * 
 */
ItemStack::ItemStack()
    :item( 0, "Air") 
{
    this->quantity = 0;
}

/**
 *
 */
ItemStack::ItemStack( const Item &item, int s )
    :item(item)
{
    this->quantity = s;
}

/**
 *
 */
std::ostream& operator<<( std::ostream &outs, const ItemStack &prt){
    outs << std::right << "(" << std::setw(2) << prt.size() << ") " 
         << prt.getItem().getName();

    return outs;
}