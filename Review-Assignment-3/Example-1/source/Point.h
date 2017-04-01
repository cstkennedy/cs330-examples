#ifndef POINT_H_INCLUDED
#define POINT_H_INCLUDED

#include <iostream>

/**
 * Coordinate in 3 dimensional Cartesian space
 */
struct Point {
    double x, y, z;

    /**
     * Default Constructor
     */
    Point();

    /**
     * Construct a Point from specified
     * x, y, and z values
     */
    Point(double x, double y, double z);

    /**
     * Apply geometric scaling function
     */
    void scale(double scalingFactor);

    /**
     * Print a point
     */
    void display(std::ostream& outs) const;
};

/**
 * Stream insertion (output) operator
 */
inline
std::ostream& operator<<(std::ostream& outs, const Point prt)
{
    prt.display(outs);
    return outs;
}

#endif