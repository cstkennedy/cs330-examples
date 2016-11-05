#include <iomanip>

#include "Item.h"

/**
 *
 */
Item::Item(){
    this->name      = "Air";
    this->stackable = true;
}

/**
 *
 */
Item::Item( std::string name ){
    this->name      = name;
    this->stackable = true;
}

Item::Item( std::string name, bool stackable ){
    this->name      = name;
    this->stackable = stackable;
}

/**
 *
 */
void Item::display( std::ostream &outs ) const{
    outs << " " << name;  
}

