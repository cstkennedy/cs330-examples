// Thomas Kennedy
// CS 330 Fall 2019

#include <iomanip>

#include "rightTriangle.h"

//------------------------------------------------------------------------------
RightTriangle::RightTriangle()
    :Triangle()
{
    _name   = "Right Triangle";

    _side_c = computeHypotenuse(_side_a, _side_b);
}

//------------------------------------------------------------------------------
RightTriangle::RightTriangle(double base, double height)
{
    this->_name   = "Right Triangle";

    this->_side_a = base;
    this->_side_b = height;
    this->_side_c = RightTriangle::computeHypotenuse(_side_a, _side_b);
}

//------------------------------------------------------------------------------
void RightTriangle::display(std::ostream &outs) const
{
    Triangle::Shape::display(outs);

    outs << std::left << std::setw(WIDTH_LABEL) << "Base"        << ": "
         << std::right << std::setw(WIDTH_VALUE) << base()       << "\n"
         << std::left << std::setw(WIDTH_LABEL) << "Height"      << ": "
         << std::right << std::setw(WIDTH_VALUE) << height()     << "\n"
         << std::left << std::setw(WIDTH_LABEL) << "Hypotenuse"  << ": "
         << std::right << std::setw(WIDTH_VALUE) << hypotenuse() << "\n"
         << std::left << std::setw(WIDTH_LABEL) << "Perimeter"   << ": "
         << std::right << std::setw(WIDTH_VALUE) << perimeter()  << "\n"
         << std::left << std::setw(WIDTH_LABEL) << "Area"        << ": "
         << std::right << std::setw(WIDTH_VALUE) << area()       << "\n";
}

//------------------------------------------------------------------------------
void RightTriangle::read(std::istream& ins)
{
    ins >> this->_side_a;  // base;
    ins >> this->_side_b;  // height;

    this->_side_c = RightTriangle::computeHypotenuse(_side_a, _side_b);
}
