#include "Consumable.h"

/**
 * 
 */
Consumable::Consumable()
    : Item( std::string(), true )
{
    this->uses = 0;
}

/**
 * 
 */
Consumable::Consumable( const Consumable &src )
    : Item( src.name, true ),
    effect( src.effect )
{
    this->uses = src.uses;
}

/**
 *
 */
void Consumable::display( std::ostream &outs ) const{
    outs << "  Nme: " << Item::name << "\n"
         << "  Eft: " << effect     << "\n"
         << "  Use: " << uses       << "\n";
}

/**
 * 
 */
void Consumable::read( std::istream& ins ){
    ins >> Item::name
        >> effect
        >> uses;
}

Item* Consumable::clone() const{
    return new Consumable( *this );
}