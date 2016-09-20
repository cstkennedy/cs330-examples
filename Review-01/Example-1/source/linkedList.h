#ifndef LL_H_INCLUDED
#define LL_H_INCLUDED

#include <iostream>
#include <iomanip>

#include "node.h"

/**
 * The LinkedList (LL) is a wrapper for three items.
 *  - Head pointer
 *  - Tail pointer
 *  - Node counter (cardinality)
 *
 * Only the head pointer is necessary, the latter three items are 
 * included for convenience.
 * <p>
 * In this version, the LinkedList has been converted to a proper class
 */
class LinkedList {
    private:
        Node *head; ///< Pointer to the first node in the LL
        Node *tail; ///< Pointer to the last node in the LL

        int nodes;  ///< Number of nodes in the LL (cardinality).

    public:
        /**
         * Construct an empty Linked List
         */
        LinkedList();

        /**
         * Traverse the Linked List while deleting each Node
         */
        ~LinkedList();

        /**
         * Add a Node at the beginning of the list
         */
        void prependNode( int to_add );

        /**
         * Add a Node at the end of the list
         */
        void appendNode( int to_add );

        /**
         * Return the number of nodes
         */
        int size() const;

        /**
         * Output the content stored within each node
         */
        void display(std::ostream& outs) const;
};

/**
 * Iterate through the LinkedList and print each individual Node
 * with an index.
 */
inline
std::ostream& operator<<( std::ostream &outs, const LinkedList &prt )
{   
    prt.display(outs);

    return outs;
}

#endif