// Thomas Kennedy
// CS 330 Fall 2014

#ifndef EQUILATERAL_TRIANGLE_H_INCLUDED
#define EQUILATERAL_TRIANGLE_H_INCLUDED

#include <iostream>
#include <string>
#include <cmath>

#include "triangle.h"

/**
 * A Triangle with all sides set to the same length
 */
class EquilateralTriangle: public Triangle{   
    private:
        static const double ROOT_3_DIV_4; ///< @f$ \frac{\sqrt{3}}{4} @f$

    public:
        /*
         * Construct an EquilateralTriangle
         * with all sides set to 1.
         */
        EquilateralTriangle();

        /*
         * Construct an EquilateralTriangle
         *
         * @param side the desired side length
         */
        EquilateralTriangle( double side );

        /*
         * Construct an EquilateralTriangle
         *
         * @param src the EquilateralTriangle to copy       
         */
        EquilateralTriangle( const EquilateralTriangle &src );

        /*
         * Deconstruct the EquilateralTriangle
         */
        virtual ~EquilateralTriangle();

        /*
         * Compute the height using
         * @f$ height = \frac{5}{4}side @f$
         *
         * @return height
         */
        double height() const;

        /*
         * Return the length of one side
         *
         * @return the length of one side
         */
        double side() const;

        /*
         * Modify the side length
         *
         * @param s the desired side length
         */
        double side( double s );

        /*
         * Compute the area using 
         * @f$ Area=@frac{\sqrt{3}}{4}side^2 @f$
         *
         * @return the area
         */
        virtual double area() const;

        /*
         * Return a new duplicate EquilateralTriangle
         */
        virtual Shape* clone() const;

        /*
         * Print the EquilateralTriangle
         */
        virtual void display( std::ostream &outs ) const;
};

/**
 *
 */
inline 
double EquilateralTriangle::height() const
{
    return sqrt( 
          1.25 * ( side() * side() )
    );
}

/**
 *
 */
inline 
double EquilateralTriangle::side() const
{
    return _side_a;
}

/**
 *
 */
inline 
double EquilateralTriangle::side( double s )
{
    _side_a = s;
    _side_b = s;
    _side_c = s;
}

/**
 *
 */
inline 
double EquilateralTriangle::area() const
{
    return ROOT_3_DIV_4 * side() * side();
}

/**
 *
 */
inline 
Shape* EquilateralTriangle::clone() const
{
    return new EquilateralTriangle( *this );
}

#endif