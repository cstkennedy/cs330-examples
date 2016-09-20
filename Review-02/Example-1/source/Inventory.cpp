#include <utility>
#include "Inventory.h"

// Allow the compiler to define the remaining 
// comparison operators
using namespace std::rel_ops;

/**
 * 
 */
Inventory::Node::Node()
    :data( Item(0, "Air"), 0 )
{
    this->next = nullptr;
}

/**
 *
 */
Inventory::Node::Node( ItemStack s )
    :data(s)
{
    this->next = nullptr;
}

/**
 *
 */
Inventory::Inventory(){
    this->first    = nullptr;
    this->last     = nullptr;
    this->slots    = 10;
    this->occupied = 0;

    //std::cout << Node().data << "\n";
}

/**
 *
 */
Inventory::Inventory( int n ){
    this->first    = nullptr;
    this->last     = nullptr;
    this->slots    = n;
    this->occupied = 0;
}

/**
 * 
 */
Inventory::Inventory( const Inventory &src ){
    this->first    = nullptr;
    this->last     = nullptr;

    this->slots    = src.slots;
    this->occupied = 0;

    for( Node *it = src.first; it!= nullptr; it = it->next ){
        this->addItems( it->data );
    }
}

/**
 *
 */
Inventory::~Inventory(){
    Node *it = first;

    while( it!= nullptr ){
        Node *next = it->next;
        delete it;

        it = next;
    }
}

/**
 *
 */
bool Inventory::addItems( ItemStack stack ){
    Node* it = first;

    // Add the first item stack
    if( it == nullptr ){
        first = new Node( stack );
        last = first;
        occupied++;

        return true;
    }

    // search for a matching stack
    while( (it != nullptr) && (it->data != stack) ){
        it = it->next;
    }

    if( it == nullptr && occupied != slots ){
        last->next = new Node( stack );
        last = last->next;

        occupied++;

        return true;
    }

    if( it != nullptr ){
        (it->data).addItems( stack.size() );

        return true;
    }

    return false;    
}

void Inventory::display( std::ostream &outs ) const{
    outs << " -Used " << occupied << " of " << slots << " slots" << "\n";

    for( Node *it = first; it!= nullptr; it = it->next ){
        outs << "  " << it->data << "\n";
    }
}

Inventory& Inventory::operator=( const Inventory &rhs ){
    if( this != &rhs ){
        Node *it = first;

        while( it!= nullptr ){
            Node *next = it->next;
            delete it;

            it = next;
        }

        this->first    = nullptr;
        this->last     = nullptr;

        this->slots    = rhs.slots;
        this->occupied = 0;

        for( Node *it = rhs.first; it!= nullptr; it = it->next ){
            this->addItems( it->data );
        }
    }

    return *this;
}