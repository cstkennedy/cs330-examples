#include "Armour.h"

/**
 *
 */
Armour::Armour()
    :Item( std::string(), false ),
    material(), modifier() 
{
    this->durability    = 0;      
    this->defense       = 0;
    //this->material      = std::string();  
    //this->modifier      = std::string();   
    this->modifer_level = 1;     
}

/**
 *
 */
Armour::Armour( const Armour &src )
    :Item( src.name, false )
{
    this->durability    = src.durability;      
    this->defense       = src.defense;        
    this->material      = src.material;  
    this->modifier      = src.modifier;   
    this->modifer_level = src.modifer_level;      
    this->element       = src.element;   
}

/**
 * 
 */
void Armour::display( std::ostream &outs ) const{
    outs << "  Nme: " << Item::name << "\n"
         << "  Dur: " << durability << "\n"
         << "  Def: " << defense    << "\n"
         << "  Mtl: " << material   << "\n"
         << "  Mdr: " << modifier   << " (Lvl " << modifer_level << ")" << "\n"
         << "  Emt: " << element    << "\n";
}

/**
 *
 */
void Armour::read( std::istream& ins ){
    ins >> Item::name
        >> material
        >> durability
        >> defense
        >> modifier
        >> modifer_level
        >> element;
}

/**
 *
 */
Item* Armour::clone() const{
    return new Armour( *this );
}

