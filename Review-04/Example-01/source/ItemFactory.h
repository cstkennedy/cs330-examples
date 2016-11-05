#ifndef ITEMFACTORY_H_INCLUDED
#define ITEMFACTORY_H_INCLUDED

#include <iostream>

#include "Item.h"
#include "Armour.h"
#include "Tool.h"
#include "Consumable.h"

/**
 * The Item Creating Wizard
 */
class ItemFactory{
    private:
        /**
         * Name Item Pair 2-tuple( type, model )
         */
        struct ItemPair{            
            std::string _type; ///< Name of the item to clone
            Item      *_model; ///< Model of the item to clone

            /**
             * Default Constructor - Used as sentinel
             */
            ItemPair();

            /**
             * Non-Default Constructor
             *
             * @param type the type of anqq item
             * @param item a cloneable item
             */
            ItemPair( std::string type, Item *item );

            /**
             * Deconstruct a ItemPair
             */
            ~ItemPair();
        };

        static ItemPair _known_items[];  ///< Listing of known items

    public:
        /**
         * Create a Item
         *
         * @param type the item to be created
         *
         * @return A item with the specified type
         *     or nullptr if no matching item is found
         */
        static Item* createItem( std::string type );

        /**
         * Determine whether a given item is known
         *
         * @param type the item for which to query
         */
        static bool isKnown( std::string type );
};

/**
 * Create the appropriate Item class--e.g., Tool, Armour or Consumable.
 *
 * How is **inheritance** used?
 */
std::istream& operator>>( std::istream &ins, Item *&rd );

#endif