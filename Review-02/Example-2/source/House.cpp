#include "House.h"

/**
 *
 */
House::House()
    :name("House"),
     rooms()
{
}

/**
 *
 */
House::House(std::string name)
    :name(name)
{
    //this->name = name;
}

/**
 *
 */
void House::addRoom(Room toAdd)
{
    rooms.push_back(toAdd);
}

/**
 *
 */
void House::display(std::ostream& outs) const
{
    outs << "--------" << this->name << "--------" << "\n";

    // All of the loops in this function are 
    // functionally equivalent
    
    for (int i = 0; i < rooms.size(); i++) {
        outs << rooms[i];
    }

    std::vector<Room>::const_iterator it = rooms.begin(); 

    while (it < rooms.end()) {
        //outs << *it;
        it->display(outs);
        it++;
    }

    for (std::vector<Room>::const_iterator it = rooms.begin(); it < rooms.end(); it++){
        outs << *it;
        //it->display(outs);
    }
    

    for (const Room& prtRoom : rooms) {
        outs << prtRoom;
    }
}