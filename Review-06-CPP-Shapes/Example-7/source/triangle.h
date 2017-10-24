// Thomas Kennedy
// CS 330 Fall 2014

#ifndef TRIANGLE_H_INCLUDED
#define TRIANGLE_H_INCLUDED

#include <iostream>
#include <string>
#include <cmath>

#include "shape.h"

/**
 * A Polygon with 3 Sides
 */
class Triangle: public Shape{
    protected:
        double _side_a;  ///< Length of Side A
        double _side_b;  ///< Length of Side B
        double _side_c;  ///< Length of Side C

    public:
        /**
         * Create a Triangle with side of length 1
         */
        Triangle();

        /**
         * Construct a triangle
         *
         * @param _side_a the length of side A
         * @param _side_b the length of side B
         * @param _side_c the length of side C
         *
         */
        Triangle( double _side_a, double _side_b, double _side_c );

        /**
         * Construct a Triangle
         *
         * @param src the Triangle to copy       
         */ 
        Triangle( const Triangle &src );

        /**
         * Deconstruct a Triangle
         */
        virtual ~Triangle();

        /**
         * Length of side A
         *
         * @return the length of side A
         */
        double sideA() const;

        /**
         * Modify the length of side A
         *
         * @param side the replacement length
         */
        void sideA( double side );

        /**
         * Length of side B
         *
         * @return the length of side B
         */
        double sideB() const;

        /**
         * Modify the length of side B
         *
         * @param side the replacement length
         */
        void sideB( double side );

        /**
         * Length of side C
         *
         * @return the length of side C
         */
        double sideC() const;

        /**
         * Modify the length of side C
         *
         * @param side the replacement length
         */
        void sideC( double side );

        /**
         * Compute the area using Heron's Formula.
         * Use @f$ s = \frac{1}{2}Perimeter @f$ and
         * @f$ Area = \sqrt{ s(s-a)(s-b)(s-c) } @f$
         *
         * @return the area
         */
        virtual double area() const;

        /**
         * Compute the perimeter
         * @f$ side_a + side_b + side_c @f$
         *
         * @return the perimeter
         */
        virtual double perimeter() const;

        /**
         * Return a new duplicate Triangle
         */
        virtual Shape* clone() const;

        /**
         * Print the Triangle
         */
        virtual void display( std::ostream &outs ) const;

        /**
         * Read the Triangle
         *
         * @param ins the input stream--i.e., source
         */
        virtual void read(std::istream &ins);
};

/**
 *
 */
inline 
double Triangle::sideA() const
{
    return _side_a;
}

/**
 *
 */
inline 
void Triangle::sideA( double side )
{
    this->_side_a = side;
}

/**
 *
 */
inline 
double Triangle::sideB() const
{
    return _side_b;
}

/**
 *
 */
inline 
void Triangle::sideB( double side )
{
    this->_side_b = side;
}

/**
 *
 */
inline 
double Triangle::sideC() const
{
    return _side_c;
}

/**
 *
 */
inline 
void Triangle::sideC( double side )
{
    this->_side_c = side;
}

/**
 *
 */
inline 
double Triangle::perimeter() const
{
    return (
        _side_a + _side_b + _side_c
    );
}

/**
 *
 */
inline 
Shape* Triangle::clone() const
{
    return new Triangle( *this );
}

#endif