// Thomas Kennedy
// CS 330 Fall 2014

#include <iomanip>

#include "equilateralTriangle.h"

const double EquilateralTriangle::ROOT_3_DIV_4 = sqrt( 3 ) / 4;

/**
 *
 */
EquilateralTriangle::EquilateralTriangle()
    :Triangle()
{
    _name = "Equilateral Triangle";
    
    side( 1 );
}

/**
 *
 */
EquilateralTriangle::EquilateralTriangle( double s )
{
    _name = "Equilateral Triangle";

    this->side( s );    
}

/**
 *
 */
EquilateralTriangle::EquilateralTriangle( const EquilateralTriangle &src )
{
    this->_name = src._name;

    this->side( src.side() );
}

/**
 *
 */
EquilateralTriangle::~EquilateralTriangle()
{
}

/**
 *
 */
void EquilateralTriangle::display( std::ostream &outs ) const
{
    Triangle::Shape::display(outs);

    outs << std::left << std::setw( WIDTH_LABEL ) << "Side"       << ": " << std::right << std::setw( WIDTH_VALUE ) << side()       << "\n"
         << std::left << std::setw( WIDTH_LABEL ) << "Height"     << ": " << std::right << std::setw( WIDTH_VALUE ) << height()     << "\n"
         << std::left << std::setw( WIDTH_LABEL ) << "Perimeter"  << ": " << std::right << std::setw( WIDTH_VALUE ) << perimeter()  << "\n"
         << std::left << std::setw( WIDTH_LABEL ) << "Area"       << ": " << std::right << std::setw( WIDTH_VALUE ) << area()       << "\n";
}
