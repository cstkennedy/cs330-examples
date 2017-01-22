#ifndef NODE_H_INCLUDED
#define NODE_H_INCLUDED

#include <iostream>
#include <iomanip>

/**
 * The Node class is the atom--smallest base component--of a Linked List.
 * An array element can viewed as its analog.
 * <p>
 * For simplicity in this example, the Node will store an integers as
 * data. In a later example, methods for generalizing this--through use of
 * templates--will be utilized.
 */
struct Node {
    int   data;  ///< Stored information
    Node* next;  ///< Link to the next Node

    /**
     * Construct an empty Node with data set to zero.
     */
    Node();

    /**
     * Construct a node given a piece of input data
     */
    Node( int data );

    /** 
     * Compare two Nodes. Check for equivalence.
     */
    bool operator==( const Node &rhs ) const;

    /** 
     * Compare two Nodes. Check for non-equivalence.
     */
    bool operator!=( const Node &rhs ) const;
};

#endif