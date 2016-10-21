// Thomas Kennedy
// CS 330 Fall 2014

#include <iomanip>

#include "triangle.h"

/**
 *
 */
Triangle::Triangle()
    :Shape( "Triangle" )
{
    this->_side_a = 1;
    this->_side_b = 1;
    this->_side_c = 1;
}

/**
 *
 */
Triangle::Triangle( double _side_a, double _side_b, double _side_c )
    :Shape( "Triangle" )
{
    this->_side_a = _side_a;
    this->_side_b = _side_b;
    this->_side_c = _side_c;
}

/**
 *
 */
Triangle::Triangle( const Triangle &src )
    :Shape( "Triangle" )
{
    this->_side_a = src._side_a;
    this->_side_b = src._side_b;
    this->_side_c = src._side_c;
}

/**
 *
 */
Triangle::~Triangle()
{
}

/**
 *
 */
double Triangle::area() const
{
    double s = perimeter() / 2;

    return (
        sqrt( 
            s * ( s - sideA() ) * ( s - sideB() ) * ( s - sideC() )
        )
    );
}

/**
 *
 */
void Triangle::display( std::ostream &outs ) const
{
    Shape::display( outs );

    outs << std::left  << std::setw( WIDTH_LABEL ) 
                       << "Side A"    << ": " 
         << std::right << std::setw( WIDTH_VALUE ) 
                       << sideA() << "\n"
         << std::left  << std::setw( WIDTH_LABEL ) 
                       << "Side B"    << ": " 
         << std::right << std::setw( WIDTH_VALUE ) 
                       << sideB() << "\n"
         << std::left  << std::setw( WIDTH_LABEL ) 
                       << "Side C" << ": " 
         << std::right << std::setw( WIDTH_VALUE ) 
                       << sideC() << "\n"
         << std::left  << std::setw( WIDTH_LABEL ) 
                       << "Perimeter" << ": " 
         << std::right << std::setw( WIDTH_VALUE ) 
                       << perimeter() << "\n"
         << std::left  << std::setw( WIDTH_LABEL ) 
                       << "Area" << ": " 
         << std::right << std::setw( WIDTH_VALUE ) 
                       << area()
         << "\n";
}