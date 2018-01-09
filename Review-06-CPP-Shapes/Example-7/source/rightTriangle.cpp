// Thomas Kennedy
// CS 330 Fall 2014

#include <iomanip>

#include "rightTriangle.h"

const double RightTriangle::ONE_HALF = ( 1.0 / 2.0 );

/**
 *
 */
RightTriangle::RightTriangle()
    :Triangle()
{
    _name   = "Right Triangle";    

    _side_c = computeHypotenuse(
        _side_a,
        _side_b
    );
}

/**
 *
 */
RightTriangle::RightTriangle( double base, double height )
{
    this->_name   = "Right Triangle";

    this->_side_a = base;
    this->_side_b = height;
    this->_side_c = RightTriangle::computeHypotenuse(
        _side_a,
        _side_b
    );
}

/**
 *
 */
RightTriangle::RightTriangle( const RightTriangle &src )
{
    this->_name   = src._name;

    this->_side_a = src._side_a;
    this->_side_b = src._side_b;
    this->_side_c = src._side_c;
}

/**
 *
 */
RightTriangle::~RightTriangle()
{
}

/**
 *
 */
void RightTriangle::display( std::ostream &outs ) const
{
    Triangle::Shape::display(outs);

    outs << std::left << std::setw( WIDTH_LABEL ) << "Base"       << ": " << std::right << std::setw( WIDTH_VALUE ) << base()       << "\n"
         << std::left << std::setw( WIDTH_LABEL ) << "Height"     << ": " << std::right << std::setw( WIDTH_VALUE ) << height()     << "\n"
         << std::left << std::setw( WIDTH_LABEL ) << "Hypotenuse" << ": " << std::right << std::setw( WIDTH_VALUE ) << hypotenuse() << "\n"
         << std::left << std::setw( WIDTH_LABEL ) << "Perimeter"  << ": " << std::right << std::setw( WIDTH_VALUE ) << perimeter()  << "\n"
         << std::left << std::setw( WIDTH_LABEL ) << "Area"       << ": " << std::right << std::setw( WIDTH_VALUE ) << area()       << "\n";
}

/**
 *
 */
void RightTriangle::read(std::istream& ins)
{
    ins >> this->_side_a; // base;
    ins >> this->_side_b; // height;

    this->_side_c = RightTriangle::computeHypotenuse(
        _side_a,
        _side_b
    );
}