// Thomas Kennedy
// CS 330 Fall 2019

#include <iomanip>

#include "triangle.h"

//------------------------------------------------------------------------------
Triangle::Triangle()
    :Triangle(1, 1, 1)
{
}

//------------------------------------------------------------------------------
Triangle::Triangle(double _side_a, double _side_b, double _side_c)
    :Shape("Triangle")
{
    this->_side_a = _side_a;
    this->_side_b = _side_b;
    this->_side_c = _side_c;
}

//------------------------------------------------------------------------------
double Triangle::area() const
{
    const double s = perimeter() / 2;

    return sqrt(s * (s - sideA())
                  * (s - sideB())
                  * (s - sideC()));
}

//------------------------------------------------------------------------------
void Triangle::display(std::ostream &outs) const
{
    Shape::display(outs);

    outs << std::left  << std::setw(WIDTH_LABEL)
                       << "Side A"    << ": "
         << std::right << std::setw(WIDTH_VALUE)
                       << sideA() << "\n"
         << std::left  << std::setw(WIDTH_LABEL)
                       << "Side B"    << ": "
         << std::right << std::setw(WIDTH_VALUE)
                       << sideB() << "\n"
         << std::left  << std::setw(WIDTH_LABEL)
                       << "Side C" << ": "
         << std::right << std::setw(WIDTH_VALUE)
                       << sideC() << "\n"
         << std::left  << std::setw(WIDTH_LABEL)
                       << "Perimeter" << ": "
         << std::right << std::setw(WIDTH_VALUE)
                       << perimeter() << "\n"
         << std::left  << std::setw(WIDTH_LABEL)
                       << "Area" << ": "
         << std::right << std::setw(WIDTH_VALUE)
                       << area()
         << "\n";
}

//------------------------------------------------------------------------------
void Triangle::read(std::istream& ins)
{
    ins >> this->_side_a;
    ins >> this->_side_b;
    ins >> this->_side_c;
}
