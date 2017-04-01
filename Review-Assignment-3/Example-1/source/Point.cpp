#include "Point.h"

/**
 *
 */
Point::Point()
    :x(0), y(0), z(0)
{
}

/**
 *
 */
Point::Point(double x, double y, double z)
{
    this-> x = x;
    this-> y = y;
    this-> z = z;
}

/**
 *
 */
void Point::scale(double scalingFactor)
{
    x *= scalingFactor;
    y *= scalingFactor;
    z *= scalingFactor;
}

/**
 *
 */
void Point::display(std::ostream& outs) const
{
    outs << "("  << x 
         << ", " << y 
         << ", " << z 
         << ")";
}