#include <iomanip>

#include "Room.h"
#include "utilities.h"

using std::cout;
using std::setw;
using std::left;
using std::right;

const std::string Room::UNITS = "ft";

//------------------------------------------------
/*
 Pay close attention to scope. Inner classes
 exist within the scope of the outer--i.e., 
 containing class
*/
//------------------------------------------------

/**
 * Default Constructor
 */
Room::Flooring::Flooring()
    :unitCost(1.00),
     type("Generic")
{
}

/**
 * Non-Default Constructor
 */
Room::Flooring::Flooring(std::string n, Cost c)
    :unitCost(c),
     type(n)
{
}

/**
 * Default to dimensions of 1
 */
Room::DimensionSet::DimensionSet()
{
    length = 1;
    width  = 1;
}

/**
 * Set the length and width to user 
 * specified values
 */
Room::DimensionSet::DimensionSet(Dimension l, Dimension w)
    :length(l),
     width(w)
{
}

/**
 * 
 */
Room::Room()
    :name("Generic"),
     flooring(),
     dimensions()
{   
    //this->flooring.type     = "Generic";
    //this->flooring.unitCost = 1;
}

/**
 *
 */
Room::Room(Dimension l, Dimension w, Cost c)
    :name("Generic"),
     flooring("Generic", c),
     dimensions(l, w)
{   
    //flooring.type     = "Generic";
    //flooring.unitCost = c;
}


/**
 *
 */
Room::Room(std::string n, Dimension l, Dimension w, Cost c)
    :name(n),
     flooring("Generic", c),
     dimensions(l, w)
{
}

/**
 *
 */
Room::Room(std::string n, DimensionSet d, Cost c, std::string fn)
    :name(n),
     flooring(fn, c),
     dimensions(d)
{
}

/**
 *
 */
void Room::setDimensions(Dimension l, Dimension w)
{
    dimensions.setLength(l);
    dimensions.setWidth(w);
}

/**
 *
 */
void Room::display(std::ostream &outs) const
{
    // Print dimensions to 1 decimal place.        
    outs.precision(1);  
    outs.setf(std::ios::fixed);

    // Let us add spacing--simulate a table
    println();

    outs << "Room (" << name << ")"         << "\n";
         
    outs << left  << setw(8) << "  Length"  << ": "
         << right << setw(8) << dimensions.getLength()    
                             << " " << UNITS << "\n";

    outs << left  << setw(8) << "  Width"  << ": "
         << right << setw(8) << dimensions.getWidth()
                             << " " << UNITS  << "\n";

    outs << left  << setw(8) << "  Area"    << ": "
         << right << setw(8) << area()    << " sq " << UNITS << "\n";

    println(outs);

    // Update to two decimal places for money
    outs.precision(2);

    // Let us hard-code the left column for this portion
    // of the output
    outs << "  Flooring  : "   << flooring.type                         << "\n"
         << "  Unit Cost : $ " << right << setw(8) << flooring.unitCost << "\n"
         << "  Total Cost: $ " << right << setw(8) << flooringCost()    << "\n";
}

/**
 *
 */
bool Room::operator==(const Room &rhs) const
{
    // Note that I am directly comparing floating 
    // point values.
    return (
        this->name   == rhs.name &&
        this->area() == rhs.area()
    );
}

/**
 *
 */
bool Room::operator<(const Room &rhs) const
{
    if (this->name == rhs.name) {
        return this->area() == rhs.area();
    }

    return this->name < rhs.name;
}