#ifndef ARMOUR_H_INCLUDED
#define ARMOUR_H_INCLUDED

#include "Item.h"

/**
 * This class represents one piece of armour--as found in most video games. This includes
 * boots and helmets.
 *
 * Armour may not be stacked. All Constructors must initialize Item::stackable
 * to false.
 */
class Armour : public Item{
    private:

    protected:
        double durability;      ///< decreases each time the armour is used
        double defense;         ///< damage that is blocked
        std::string material;   ///< material out of which the armour is composed 
        std::string modifier;   ///< one of protection, feather_falling, or unbreaking
        int modifer_level;      ///< modifier level in the range 1 to 3
        std::string element;    ///< associated element--e.g., ice, fire, and lightning.

    public:
        /**
         * Default to a armour with an empty name
         */
        Armour();

        /**
         * Copy Constructor
         */
        Armour( const Armour &src );

        /**
         * Retrieve armour durability
         */
        double getDurability() const;

        /**
         * Set armour durability
         */
        void setDurability( double durability );

        /**
         * Retrieve armour defense
         */
        double getDefense() const;

        /**
         * Set armour defense
         */
        void setDefense( double defense );       

        /**
         * Retrieve armour Material
         */
        std::string getMaterial() const;

        /**
         * Set armour Material
         */
        void setMaterial( std::string m );

        /**
         * Retrieve armour Modifier
         */
        std::string getModifier() const;

        /**
         * Set armour Modifier
         */
        void setModifier( std::string m );

        /**
         * Retrieve armour Modifier Level
         */
        double getModiferLevel() const;

        /**
         * Set armour Modifier Level
         */
        void setModiferLevel( double level );

        /**
         * Retrieve armour Element
         */
        std::string getElement() const;

        /**
         * Set armour Element
         */
        void setElement( std::string e );

        /**
         * Print one Armour
         */
        virtual void display( std::ostream &outs ) const;

        /**
         * Read Armour attributes from an input stream
         */
        virtual void read( std::istream& ins );

        /**
         * Clone--i.e., copy--this Armour
         */
        virtual Item* clone() const;
};

/**
 *
 */
inline double Armour::getDurability() const{
    return this->durability;
}

/**
 *
 */
inline void Armour::setDurability( double durability ){
    this->durability;
}

/**
 * 
 */
inline double Armour::getDefense() const{
    return this->defense;
}

/**
 * 
 */
inline void Armour::setDefense( double defense ){
    this->defense = defense;
}   

/**
 *
 */
inline std::string Armour::getMaterial() const{
    return this->material;
}

/**
 *
 */
inline void Armour::setMaterial( std::string m ){
    this->material = m;
}

/**
 *
 */
inline std::string Armour::getModifier() const{
    return this->modifier;
}

/**
 *
 */
inline void Armour::setModifier( std::string m ){
    this->modifier = m;
}

/**
 *
 */
inline double Armour::getModiferLevel() const{
    return this->modifer_level;
}

/**
 *
 */
inline void Armour::setModiferLevel( double level ){
    this->modifer_level = level;
}

/**
 * 
 */
inline std::string Armour::getElement() const{
    return this->element;
}

/**
 * 
 */
inline void Armour::setElement( std::string e ){
    this->element = e;
}

#endif