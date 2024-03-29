#include <iomanip>

#include "Room.h"
#include "utilities.h"

using std::cout;
using std::setw;
using std::left;
using std::right;

const std::string Room::UNITS = "ft";

//------------------------------------------------------------------------------
/*
 Pay close attention to scope. Inner classes exist within the scope of
 the outer--i.e., containing class
*/
//------------------------------------------------------------------------------

Room::Flooring::Flooring()
    :Flooring("Generic", 1.00)
{
}

//------------------------------------------------------------------------------
Room::Flooring::Flooring(std::string n, Cost c)
    :type(n),
     unitCost(c)
{
}

//------------------------------------------------------------------------------
Room::DimensionSet::DimensionSet()
{
    length = 1;
    width  = 1;
}

//------------------------------------------------------------------------------
Room::DimensionSet::DimensionSet(Dimension l, Dimension w)
    :length(l),
     width(w)
{
}

//------------------------------------------------------------------------------
Room::Room()
    :name("Generic")
{
    this->dimensions = new DimensionSet();
    this->flooring = new Flooring();
}

//------------------------------------------------------------------------------
Room::Room(Dimension l, Dimension w, Cost c)
    :Room("Generic", l, w, c)
{
}


//------------------------------------------------------------------------------
Room::Room(std::string n, Dimension l, Dimension w, Cost c)
    :name(n)
{
     this->flooring =  new Flooring("Generic", c);
     this->dimensions =  new DimensionSet(l, w);
}

//------------------------------------------------------------------------------
Room::Room(std::string n, DimensionSet d, Cost c, std::string fn)
     :name(n)
{
     this->flooring =  new Flooring(fn, c);
     this->dimensions =  new DimensionSet(d);
}
        
//------------------------------------------------------------------------------
Room::Room(const Room& src)
{
    this->name = src.name;
    this->flooring = new Flooring(*(src.flooring));
    this->dimensions = new DimensionSet(*src.dimensions);
}

//------------------------------------------------------------------------------
Room::~Room()
{
    delete flooring;
    delete dimensions;
}

//------------------------------------------------------------------------------
Room& Room::operator=(Room rhs)
{
    std::swap(this->name, rhs.name);
    std::swap(this->dimensions, rhs.dimensions);
    std::swap(this->flooring, rhs.flooring);

    return *this;
}

//------------------------------------------------------------------------------
void Room::setDimensions(Dimension l, Dimension w)
{
    dimensions->setLength(l);
    dimensions->setWidth(w);
}

//------------------------------------------------------------------------------
void Room::display(std::ostream& outs) const
{
    // Print dimensions to 1 decimal place.
    outs.precision(1);
    outs.setf(std::ios::fixed);

    // Let us add spacing--simulate a table
    println(outs);

    outs << "Room (" << name << ")"         << "\n";

    outs << left  << setw(8) << "  Length"  << ": "
         << right << setw(8) << dimensions->getLength()
                             << " " << UNITS << "\n";

    outs << left  << setw(8) << "  Width"  << ": "
         << right << setw(8) << dimensions->getWidth()
                             << " " << UNITS  << "\n";

    outs << left  << setw(8) << "  Area"    << ": "
         << right << setw(8) << area()    << " sq " << UNITS << "\n";

    println(outs);

    // Update to two decimal places for money
    outs.precision(2);

    // Let us hard-code the left column for this portion
    // of the output
    outs << "  Flooring  : "   << flooring->type                         << "\n"
         << "  Unit Cost : $ " << right << setw(8) << flooring->unitCost << "\n"
         << "  Total Cost: $ " << right << setw(8) << flooringCost()    << "\n";
}

//------------------------------------------------------------------------------
bool Room::operator==(const Room& rhs) const
{
    // Note that I am directly comparing floating
    // point values.
    return this->name == rhs.name
        && this->area() == rhs.area();
}

//------------------------------------------------------------------------------
bool Room::operator<(const Room& rhs) const
{
    if (this->name == rhs.name) {
        return this->area() < rhs.area();
    }

    return this->name < rhs.name;
}
