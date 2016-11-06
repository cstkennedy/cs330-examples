#ifndef TOOL_H_INCLUDED
#define TOOL_H_INCLUDED

#include "Item.h"

/**
 * This class represents one tool--as found in most video games. This includes
 * pickaxes and shovels.
 *
 * Tools may not be stacked. All Constructors must initialize Item::stackable
 * to false.
 */
class Tool : public Item{
    private:

    protected:
        double      durability;    ///< decreases each time the tool is used
        double      speed;         ///< base harvest/mine speed
        std::string material;      ///< material out of which the tool is composed 
        std::string modifier;      ///< one of efficiency, fortune, or unbreaking
        int         modifer_level; ///< modifier level in the range 1 to 5

    public:
        /**
         * Default to a tool with an empty name
         */
        Tool();

        /**
         * Copy Constructor
         */
        Tool( const Tool &src );

        /**
         * Retrieve tool durability
         */
        double getDurability() const;

        /**
         * Set tool durability
         */
        void setDurability( double durability );

        /**
         * Retrieve tool speed
         */
        double getSpeed() const;

        /**
         * Set tool speed
         */
        void setSpeed( double speed );

        /**
         * Retrieve tool Material
         */
        std::string getMaterial() const;

        /**
         * Set tool Material
         */
        void setMaterial( std::string m );

        /**
         * Retrieve tool Modifier
         */
        std::string getModifier() const;

        /**
         * Set tool Modifier
         */
        void setModifier( std::string m );

        /**
         * Retrieve tool Modifier Level
         */
        double getModiferLevel() const;

        /**
         * Set tool Modifier Level
         */
        void setModiferLevel( double level );

        /**
         * Print one Tool
         */
        virtual void display( std::ostream &outs ) const;

        /**
         * Read Tool attributes from an input stream
         */
        virtual void read( std::istream& ins );

        /**
         * Clone--i.e., copy--this Tool
         */
        virtual Item* clone() const;
};

/**
 *
 */
inline double Tool::getDurability() const{
    return this->durability;
}

/**
 *
 */
inline void Tool::setDurability( double durability ){
    this->durability;
}

/**
 *
 */
inline double Tool::getSpeed() const{
    return this->speed;
}

/**
 *
 */
inline void Tool::setSpeed( double speed ){
    this->speed = speed;
}

/**
 *
 */
inline std::string Tool::getMaterial() const{
    return this->material;
}

/**
 *
 */
inline void Tool::setMaterial( std::string m ){
    this->material = m;
}

/**
 *
 */
inline std::string Tool::getModifier() const{
    return this->modifier;
}

/**
 *
 */
inline void Tool::setModifier( std::string m ){
    this->modifier = m;
}

/**
 *
 */
inline double Tool::getModiferLevel() const{
    return this->modifer_level;
}

/**
 *
 */
inline void Tool::setModiferLevel( double level ){
    this->modifer_level = level;
}

#endif