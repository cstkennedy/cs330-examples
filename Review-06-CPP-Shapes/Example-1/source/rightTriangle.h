// Thomas Kennedy
// CS 330 Fall 2014

#ifndef RIGHT_TRIANGLE_H_INCLUDED
#define RIGHT_TRIANGLE_H_INCLUDED

#include <iostream>
#include <string>
#include <cmath>

#include "triangle.h"

/**
 *
 */
class RightTriangle: public Triangle{   
    private:
        static const double ONE_HALF; ///< @f$ \frac{1}{2} @f$

        /**
         * Compute the hyptoenuse using: 
         * @f$ hypotenuse = \sqrt{base^2 + height^2} @f$
         *
         * @param base the base of a Right Triangle
         * @param height the height of a Right Triangle
         *
         * @return the hypotenuse of a right triangle
         */
        static double computeHypotenuse( double base, double height );

    public:
        /**
         * Construct a RightTriangle
         * with base and height set to 1.
         */
        RightTriangle();

        /**
         * Construct a RightTriangle
         *
         * @param base the desired base
         * @param height the desired height
         */
        RightTriangle( double base, double height );

        /**
         * Construct a RightTriangle
         *
         * @param src the RightTriangle to copy       
         */
        RightTriangle( const RightTriangle &src );

        /**
         * Deconstruct the RightTriangle
         */
        virtual ~RightTriangle();

        /**
         * Return the base
         */
        double base() const;

        /**
         * Modify the base
         *
         * @param side the replacement base
         */
        void base( double side );

        /**
         * Return the height
         */
        double height() const;

        /*
         * Modify the height
         *
         * @param side the replacement height
         */
        void height( double side );

        /**
         * Return the hypotenuse
         *
         * @return the hypotenuse
         */
        double hypotenuse() const;

        /**
         * Compute the area using
         * @f$ Area = \frac{1}{2}*base*height @f$
         *
         * @return the area
         */
        virtual double area() const;

        /**
         * Return a new duplicate RightTriangle
         */
        virtual Shape* clone() const;

        /**
         * Print the RightTriangle
         *
         * @param outs the output stream--i.e., destination
         */
        virtual void display( std::ostream &outs ) const;
};

/**
 *
 */
inline
double RightTriangle::computeHypotenuse(double base, double height )
{
    return sqrt(
        ( base * base ) + ( height * height )
    );
}

/**
 *
 */
inline 
double RightTriangle::base() const
{
    return _side_a;
}

/**
 *
 */
inline 
void RightTriangle::base( double side )
{
    _side_a = side;
    
    _side_c = RightTriangle::computeHypotenuse(
        _side_a,
        _side_b
    );
}

/**
 *
 */
inline 
double RightTriangle::height() const
{
    return _side_b;
}

/**
 *
 */
inline 
void RightTriangle::height( double side )
{
    _side_b = side;

    _side_c = RightTriangle::computeHypotenuse(
        _side_a,
        _side_b
    );
}

/**
 *
 */
inline 
double RightTriangle::hypotenuse() const
{
    return _side_c;
}

/**
 *
 */
inline 
double RightTriangle::area() const
{
    return ONE_HALF * _side_a * _side_b;
}

inline 
Shape* RightTriangle::clone() const
{
    return new RightTriangle( *this );
}

#endif