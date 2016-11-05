#include "ItemFactory.h"

ItemFactory::ItemPair ItemFactory::_known_items[] = {
    ItemPair( "Armour"     , new Armour()     ),
    ItemPair( "Armor"      , new Armour()     ),
    ItemPair( "Tool"       , new Tool()       ),
    ItemPair( "Food"       , new Consumable() ),
    ItemPair( "Potion"     , new Consumable() ),
    ItemPair( "Disposable" , new Consumable() ),
    ItemPair()
};

/**
 *
 */
ItemFactory::ItemPair::ItemPair()
    : _type(), _model( nullptr )
{
}

/**
 *
 */
ItemFactory::ItemPair::ItemPair( std::string type, Item *item )
    : _type( type ), _model( item )
{
}

ItemFactory::ItemPair::~ItemPair(){
    delete _model;
}

/**
 *
 */
Item* ItemFactory::createItem( std::string type ){
    for( int i = 0; _known_items[i]._model != nullptr; i++ ){
        if( _known_items[i]._type == type ){
            return _known_items[i]._model->clone();
        }
    }

    // A item with the given type could not be found
    return nullptr;
}

/**
 *
 */
std::istream& operator>>( std::istream &ins, Item *&rd ){
    std::string type; // first word  on line
    
    // Read the item type and
    // retrieve a template Item from the ItemFactory
    ins >> type;
    rd = ItemFactory::createItem( type );

    // If the item type is recognized,
    // read the item.
    // otherwise discard the remainder 
    // of the line
    if( rd != nullptr ){
        rd->read( ins );
    }
    else{
        //discard remainder of line
        std::string line;
        getline( ins, line );        
    }   

    return ins;
}