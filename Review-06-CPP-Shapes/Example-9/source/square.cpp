// Thomas Kennedy
// CS 330 Fall 2014

#include <iomanip>

#include "square.h"

//------------------------------------------------------------------------------
Square::Square()
	:Square(1)
{
}

//------------------------------------------------------------------------------
Square::Square(double s)
	:Shape("Square"),
     _side(s)
{
}

//------------------------------------------------------------------------------
Square::Square(const Square &src)
	:Square(src._side)
{
}

//------------------------------------------------------------------------------
Square::~Square()
{
}

//------------------------------------------------------------------------------
void Square::display(std::ostream &outs) const
{
    Shape::display(outs);

    outs << std::left << std::setw(WIDTH_LABEL)  << "Side"      << ": "
         << std::right << std::setw(WIDTH_VALUE) << side()      << "\n"
         << std::left << std::setw(WIDTH_LABEL) << "Perimeter"  << ": "
         << std::right << std::setw(WIDTH_VALUE) << perimeter() << "\n"
         << std::left << std::setw(WIDTH_LABEL) << "Area"       << ": "
         << std::right << std::setw(WIDTH_VALUE) << area()      << "\n";
}

//------------------------------------------------------------------------------
void Square::read(std::istream& ins)
{
    ins >> this->_side;
}
