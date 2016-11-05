#include "Tool.h"

/**
 *
 */
Tool::Tool()
    :Item( std::string(), false )
{
    this->durability    = 0;      
    this->speed         = 0;
    this->material      = std::string();  
    this->modifier      = std::string();   
    this->modifer_level = 1;     
}

/**
 *
 */
Tool::Tool( const Tool &src )
    :Item( src.name, false )
{
    this->durability    = src.durability;      
    this->speed         = src.speed;        
    this->material      = src.material;  
    this->modifier      = src.modifier;   
    this->modifer_level = src.modifer_level;      
}

/**
 * 
 */
void Tool::display( std::ostream &outs ) const{
    outs << "  Nme: " << Item::name << "\n"
         << "  Dur: " << durability << "\n"
         << "  Spd: " << speed      << "\n"
         << "  Mtl: " << material   << "\n"
         << "  Mdr: " << modifier   << " (Lvl " << modifer_level << ")" << "\n";  
}

/**
 *
 */
void Tool::read( std::istream& ins ){
    ins >> Item::name
        >> material
        >> durability
        >> speed
        >> modifier
        >> modifer_level;
}

/**
 *
 */
Item* Tool::clone() const{
    return new Tool( *this );
}

