#include <iomanip>

#include "ItemStack.h"

/**
 * 
 */
ItemStack::ItemStack(){
    this->item     = nullptr;
    this->quantity = 0;
}

/**
 *
 */
ItemStack::ItemStack( const Item *item ){
    this->item     = item->clone();
    this->quantity = 1;
}

/**
 * 
 */
ItemStack::ItemStack( const ItemStack &src ){
    this->item     = src.item->clone();
    this->quantity = src.quantity;
}

/**
 * 
 */
ItemStack::~ItemStack(){
    delete this->item;
}

/**
 *
 */
void ItemStack::addItems( int a ){
    // Add *a* items if stacking is permitted
    // otherwise, silently discard *a* items
    if( item->isStackable() ){
        this->quantity += a;
    }
}

/**
 *
 */
bool ItemStack::operator=( const ItemStack &rhs ){
    delete this->item;

    this->item     = rhs.item->clone();
    this->quantity = rhs.quantity;
}

/**
 *
 */
std::ostream& operator<<( std::ostream &outs, const ItemStack &prt ){
    prt.getItem()->display( outs );

    if( prt.permitsStacking() ){
        outs << "  Qty: " << prt.size() << "\n";
    }

    return outs;
}