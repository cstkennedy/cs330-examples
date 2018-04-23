#include "linkedList.h"

/**
 * 
 */
LinkedList::Node::Node()
{
    this->data = 0;
    this->next = nullptr;
}

/**
 * 
 */
LinkedList::Node::Node( int data )
{
    this->data = data;
    this->next = nullptr;
}

/**
 *
 */
bool LinkedList::Node::operator==( const Node &rhs ) const
{
    return (
        this->data == rhs.data &&
        this->next == rhs.next
    );
}

/**
 *
 */
bool LinkedList::Node::operator!=( const Node &rhs ) const
{
    return (
        this->data != rhs.data ||
        this->next != rhs.next
    );
}

/**
 *
 */
LinkedList::LinkedList()
{
    //Initialize the private data members (attributes)    
    head  = nullptr;
    tail  = nullptr;
    nodes = 0;
}

/**
 *
 */
LinkedList::~LinkedList()
{
    Node *this_iterator = nullptr; // Loop control pointer
    Node *to_delete     = nullptr; // Node to delete        
    
    //start at the beginning of the this
    this_iterator = this->head;
    
    //iterate through the this and delete each node
    while( this_iterator != nullptr  ){
        to_delete = this_iterator;
        
        //move to next node
        this_iterator = this_iterator->next;
       
        //delete the current node        
        delete to_delete;
        
        to_delete = nullptr; //dangling pointers are bad

        // Such output would not be included in
        // a non-academic exercise
        std::cerr << "Deleting Node" << "\n";
    }
            
    // Are these three lines necessary?
    head  = nullptr;
    tail  = nullptr;
    nodes = 0;
}

/**
 *
 */
void LinkedList::prependNode( int to_add )
{
    // Create a new Node
    // Note what actually occurs on the next line
    Node *new_node = nullptr;

    // If this is the first node
    // invoke appendNode
    //
    // What two conditions are equivalent to nodes == 0?
    // Consider how you would write this expression 
    // using head or tail
    //
    if( this->nodes == 0 ){
        appendNode( to_add );
        return;
    }    

    // Why did I wait until here to constuct
    // the Node. 
    //
    // Think about the preceding lines and dangling
    // pointers
    //
    new_node = new Node( to_add );

    // Setup the new node
    new_node->next = ( this->head );
    this->head = new_node;

    // Increase the number of nodes
    this->nodes++;

    // Do not allow access to the Node except 
    // through the linked list
    // Is this line necessary?
    new_node = nullptr;
}

/**
 *
 */
void LinkedList::appendNode( int to_add )
{
    // Create a new Node
    // Note what actually occurs on the next line
    Node *new_node = nullptr;

    // Store the "to_add" data within the node
    new_node = new Node( to_add );

    // Handle the case where the first node is added
    if( this->nodes == 0){
        // this->head = this->tail = new_node; 
        // Note the above line is it any different
        // from the next two lines?
        this->head = new_node;
        this->tail = new_node;
    }
    else{
        // Add the new node to the this
        // What happens on the following two lines
        (this->tail )->next = new_node;
        this->tail = new_node;        
    }

    // Increase the number of nodes
    this->nodes++;

    // Do not allow access to the node except 
    // through the linked list
    // Is this line necessary?
    new_node = nullptr;
}

/**
 * 
 */
int LinkedList::size() const
{
    return this->nodes;
}

/**
 *
 */
void LinkedList::display(std::ostream& outs) const
{
    int index = 0;   // Used to output ids
    Node* it  = this->head;

    while ( it != nullptr ) {
        outs << "Node # " << std::right << std::setw(4) << index 
             << " - "     << std::right << std::setw(4) << it->data             
             << "\n"; 

        // increment index
        index++;

        it = it->next;
    }
}

