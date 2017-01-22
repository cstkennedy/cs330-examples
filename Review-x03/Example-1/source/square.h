// Thomas Kennedy
// CS 330 Fall 2014

#ifndef SQUARE_H_INCLUDED
#define SQUARE_H_INCLUDED

#include <iostream>
#include <string>

#include "shape.h"

/**
 * A Rectangle with 4 Equal Sides
 */
class Square: public Shape{  
    private:
        double _side;  ///< Length of One Side

    public:
        /**
         * Construct a Square with side set to 1
         */
        Square();

        /**
         * Construct a Square
         * 
         * @param s the desired side length
         */
        Square( double s );

        /**
         * Construct a Square
         *
         * @param src the Square to copy       
         */ 
        Square( const Square &src );

        /**
         * Square Destructor
         */
        virtual ~Square();

        /**
         * Return the side length
         */
        double side() const;

        /**
         * Modify the side length
         *
         * @param s the replacement length
         */
        void side( double s );

        /**
         * Compute the area
         *
         * @return area
         */
        virtual double area() const;

        /**
         * Compute the perimeter
         *
         * @return perimeter
         */
        virtual double perimeter() const;

        /**
         * Return a new duplicate Square
         */
        virtual Shape* clone() const;

        /**
         * Print the Square
         *
         * @param outs the output stream--i.e., destination
         */
        virtual void display( std::ostream &outs ) const;
};

/**
 *
 */
inline
double Square::side() const{
    return _side;
}

/**
 *
 */
inline 
void Square::side( double s )
{
    _side = s;
}

/**
 *
 */
inline 
double Square::area() const
{
    return (
        _side * _side
    );
}

/**
 *
 */
inline 
double Square::perimeter() const
{
    return (
        4 * _side
    );
}

/**
 *
 */
inline 
Shape* Square::clone() const
{
    return new Square( *this );
}

#endif