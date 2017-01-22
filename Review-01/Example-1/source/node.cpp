#include "node.h"

/**
 * 
 */
Node::Node()
{
   this->data = 0;
   this->next = nullptr;
}

/**
 * 
 */
Node::Node( int data )
{
    this->data = data;
    this->next = nullptr;
}

/**
 *
 */
bool Node::operator==( const Node &rhs ) const
{
    return (
        this->data == rhs.data &&
        this->next == rhs.next
    );
}

/**
 *
 */
bool Node::operator!=( const Node &rhs ) const
{
    return (
        this->data != rhs.data ||
        this->next != rhs.next
    );
}
