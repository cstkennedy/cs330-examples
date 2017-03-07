// Thomas Kennedy
// CS 330 Fall 2014

#include "shape.h"

#include <iomanip>

const int Shape::WIDTH_LABEL = 12;
const int Shape::WIDTH_VALUE = 24;

/**
 *
 */
Shape::Shape()
    :_name( "Shape" )
{
}

/**
 *
 */
Shape::Shape( std::string name )
    :_name( name )
{
}

/**
 *
 */
Shape::~Shape()
{
}

/**
 *
 */
void Shape::display( std::ostream &outs ) const
{
    outs << std::left  << std::setw( WIDTH_LABEL ) 
                       << "Name" << ": " 
         << std::right << std::setw( WIDTH_VALUE ) 
                       << name() 
         << "\n";
}