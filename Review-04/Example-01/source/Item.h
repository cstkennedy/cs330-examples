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
        bool stackable;   ///< Indicates whether this Item can placed in an ItemStack larger than 1.

    protected:
        std::string name; ///< Short title--e.g., HP Potion

    public:
        /**
         * Default to name = Air and stackable = true
         */
        Item();

        /**
         * Create an Item with a specifiedand name
         *
         * @pre 
         *  - all items that share a name are of the same type
         *  - differences in capitalization denote different items 
         */
        Item( std::string name );

        /**
         * Create an Item with a specified id and name
         *
         * @param stackable flag that indicates whether duplicates
         *      of this item can be stacked
         *
         * @pre 
         *  - all items that share a name are of the same type
         *  - differences in capitalization denote different items
         */
        Item( std::string name, bool stackable );

        /**
         * Retrieve name
         */
        std::string getName() const;

        /**
         * Update name
         */
        void setName( std::string n );

        /**
         * Check for logical equivalence--based on name
         */
        bool operator==( const Item &rhs ) const;

        /**
         * Check ordering--based on name
         */
        bool operator<( const Item &rhs ) const;

        /**
         * Can this item be placed in a stack?
         */
        bool isStackable() const;

        /**
         * Print one Item
         */
        virtual void display( std::ostream &outs ) const;

        /**
         * Read the item from an input stream
         */
        virtual void read( std::istream& ins ) = 0;

        /**
         * Copy this Item--i.e., duplicate this Item
         */
        virtual Item* clone() const = 0;
};

/**
 * Print one Item by invoking display--through dynamic binding
 */
std::ostream& operator<<( std::ostream &outs, const Item &prt );

/**
 * 
 */
inline
std::string Item::getName() const{
    return this->name;
}

/**
 * 
 */
inline
void Item::setName( std::string n ){
    this->name = n;
}

/**
 *
 */
inline
bool Item::isStackable() const{
    return this->stackable;
}

/**
 *
 */
inline
bool Item::operator==( const Item &rhs ) const{
    return this->name == rhs.name;
}

/**
 *
 */
inline
bool Item::operator<( const Item &rhs ) const{
    return this->name < rhs.name;
}

/**
 *
 */
inline
std::ostream& operator<<( std::ostream &outs, const Item &prt ){
    prt.display( outs );

    return outs;
}

#endif