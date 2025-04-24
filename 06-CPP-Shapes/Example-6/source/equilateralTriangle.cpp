// Thomas Kennedy
// CS 330 Fall 2019

#include <iomanip>

#include "equilateralTriangle.h"

//------------------------------------------------------------------------------
EquilateralTriangle::EquilateralTriangle()
    :EquilateralTriangle(1)
{
}

//------------------------------------------------------------------------------
EquilateralTriangle::EquilateralTriangle(double s)
    :Triangle(s, s, s)
{
    _name = "Equilateral Triangle";
}

//------------------------------------------------------------------------------
void EquilateralTriangle::display(std::ostream &outs) const
{
    Triangle::Shape::display(outs);

    outs << std::left << std::setw(WIDTH_LABEL) << "Side"        << ": "
         << std::right << std::setw(WIDTH_VALUE) << side()       << "\n"
         << std::left << std::setw(WIDTH_LABEL) << "Height"      << ": "
         << std::right << std::setw(WIDTH_VALUE) << height()     << "\n"
         << std::left << std::setw(WIDTH_LABEL) << "Perimeter"   << ": "
         << std::right << std::setw(WIDTH_VALUE) << perimeter()  << "\n"
         << std::left << std::setw(WIDTH_LABEL) << "Area"        << ": "
         << std::right << std::setw(WIDTH_VALUE) << area()       << "\n";
}

//------------------------------------------------------------------------------
void EquilateralTriangle::read(std::istream& ins)
{
    double inSide;

    ins >> inSide;

    this->side(inSide);
}
