#ifndef ITEM_H_INCLUDED
#define ITEM_H_INCLUDED

#include <iostream>
#include <string>

/**
 * Item represents an individual Item in an inventory.
 * This includes items such as potions, building materials, and food.
 *
 * Only one of each item can exist--i.e., no two items share the
 * same numeric id.
 */
class Item{
    private:
        int id;           ///< Unique numeric identifier--e.g., 1
        std::string name; ///< Short title--e.g., HP Potion 

    public:
        /**
         * Default to id = 0 and name = Air 
         */
        Item();

        /**
         * Create an Item with an id and
         * a blank name
         */
        Item( int id );

        /**
         * Create an Item with a specified id and name
         *
         * @pre 
         *  - all items that share an id are of the same type
         *  - id is strictly positive
         */
        Item( int id, std::string name );

        /**
         * Retrieve numeric id
         */
        int getID() const;

        /**
         * Update numeric id  
         *
         * @pre i is strictly positive
         */
        void setID( int i );

        /**
         * Retrieve name
         */
        std::string getName() const;

        /**
         * Update name
         */
        void setName( std::string n );

        /**
         * Check for logical equivalence--based on numeric id
         */
        bool operator==( const Item &rhs ) const;

        /**
         * Check ordering--based on numeric id
         */
        bool operator<( const Item &rhs ) const;

        /**
         * Print one Item
         */
        void display( std::ostream &outs ) const;
};

/**
 * Print one Item by invoking display
 */
std::ostream& operator<<( std::ostream &outs, const Item &prt );

/**
 * 
 */
inline int Item::getID() const{
    return this->id;
}

/**
 * 
 */
inline void Item::setID( int i ){
    this->id = i;
}

/**
 * 
 */
inline std::string Item::getName() const{
    return this->name;
}

/**
 * 
 */
inline void Item::setName( std::string n ){
    this->name = n;
}

/**
 *
 */
inline bool Item::operator==( const Item &rhs ) const{
    return this->id == rhs.id;
}

/**
 *
 */
inline bool Item::operator<( const Item &rhs ) const{
    return this->id < rhs.id;
}

/**
 *
 */
inline std::ostream& operator<<( std::ostream &outs, const Item &prt ){
    prt.display( outs );

    return outs;
}

#endif