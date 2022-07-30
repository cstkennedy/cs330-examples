// Thomas Kennedy
// CS 330 Fall 2019

#include "shape.h"

#include <iomanip>

//------------------------------------------------------------------------------
Shape::Shape(std::string name)
    :_name(name)
{
}

//------------------------------------------------------------------------------
void Shape::display(std::ostream &outs) const
{
    outs << std::left  << std::setw(WIDTH_LABEL)
                       << "Name" << ": "
         << std::right << std::setw(WIDTH_VALUE)
                       << name()
         << "\n";
}
