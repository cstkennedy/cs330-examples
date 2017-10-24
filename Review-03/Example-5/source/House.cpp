#include "House.h"


/**
 *
 */
House::Node::Node(Room d)
    :data(d),
     next(nullptr)
{
}

/**
 *
 */
House::House()
    :name("House"),
     head(nullptr),
     tail(nullptr)
{
    currentSize = 0;
}

/**
 *
 */
House::House(std::string name)
    :name(name),
     head(nullptr),
     tail(nullptr)
{
    currentSize = 0;
}

/**
 *
 */
House::House(const House &src)
    :name(src.name),
     head(nullptr),
     tail(nullptr),
     currentSize(0)
{
    Node* it = src.head;

    while (it != nullptr) {
        this->addRoom(it->data);

        it = it->next;
    }
}

/**
 *
 */
House::~House() {
    // Deallocate the Linked List
    Node* it = this->head;

    while (it != nullptr) {
        Node* prev = it;

        it = it->next;
        delete prev;
    }
    it = nullptr;

    this->head = nullptr;
    this->tail = nullptr;
    // End Linked List Deallocation
}

/**
 *
 */
void House::addRoom(Room toAdd)
{
    Node* newNode = new Node(toAdd);

    // If adding the first Node
    if (head == nullptr) {
        head        = newNode;
        tail        = newNode;
        currentSize = 1;

        // Why set newNode to null?
        newNode     = nullptr;

        return;
    }

    // Link the newNode to the end
    // of the exiting list
    tail->next = newNode;

    // Update tail;
    tail = tail->next;
    //tail = newNode;

    // Update the size
    currentSize++;
}

/**
 *
 */
size_t House::size() const {
    return currentSize;
}

/**
 *
 */
House::iterator House::begin()
{
    return iterator(head);
}

/**
 *
 */
House::const_iterator House::begin() const
{
    return const_iterator(head);
}

/**
 *
 */
House::iterator House::end()
{
    return iterator(nullptr);
}

/**
 *
 */
House::const_iterator House::end() const
{
    return const_iterator(nullptr);
}

/**
 *
 */
void House::display(std::ostream& outs) const
{
    outs << "--------" << this->name << "--------" << "\n";

    // What type of iterator am I using--i.e.,
    // iterator or const_iterator?

    // Print the rooms
    for (const Room& prtRoom : *this) {
        outs << prtRoom;
    }

    // Compute and print the total
    double                avg   = 0;
    double                total = 0;

    for (const Room& room : *this) {
        total += room.flooringCost();
    }

    avg = total / this->size();

    outs << "\n";
    outs << "------------------------------";
    outs << "\n";

    outs << "Total Cost   : $ " << total << "\n";
    outs << "Avg Room Cost: $ " << avg   << "\n";
}


/**
 *
 */
House& House::operator=(House rhs)
{
    std::swap(*this, rhs);

    return *this;
}

/**
 *
 */
void swap(House& lhs, House& rhs)
{
    using std::swap;

    swap(lhs.name, rhs.name);
    swap(lhs.head, rhs.head);
    swap(lhs.tail, rhs.tail);
    swap(lhs.currentSize, rhs.currentSize);
}