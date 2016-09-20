#include <iomanip>

#include "Item.h"

/**
 *
 */
Item::Item(){
    this->id   = 0;
    this->name = "Air";
}

/**
 *
 */
Item::Item( int id )
    :name()
{
    this->id = id;
}

/**
 *
 */
Item::Item( int id, std::string name ){
    this->id   = id;
    this->name = name;
}

/**
 *
 */
void Item::display( std::ostream &outs ) const{
    outs << std::right << std::setw(3) << id << " " << name;  
}