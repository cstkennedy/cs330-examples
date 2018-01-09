// Thomas Kennedy
// CS 330 Fall 2014

#ifndef CIRCLE_H_INCLUDED
#define CIRCLE_H_INCLUDED

#include <iostream>
#include <string>

#include "shape.h"

/**
 * An Ellipse with equivalent major and minor axes
 */
class Circle: public Shape{  
    private:
        static const double PI;   ///< @f$ \pi @f$
        static const double TAU;  ///< @f$ \tau = 2\pi @f$

        double _radius;  ///< Length of the radius

    public:
        /**
         * Construct a Circle with radius set to 1
         */ 
        Circle();

        /**
         * Construct a Circle
         *  
         * @param s the desired radius length
         */ 
        Circle( double r );

        /**
         * Construct a Circle
         * 
         * @param src the Circle to copy       
         */  
        Circle( const Circle &src );

        /**
         * Circle Destructor
         */ 
        virtual ~Circle();

        /**
         * Return the radius length
         */ 
        double radius() const;

        /**
         * Modify the radius length
         * 
         * @param r the replacement length
         */ 
        void radius( double r );

        /**
         * Return the diameter
         */ 
        double diameter() const;

        /**
         * Compute the area using @f$ \pi r^2 @f$
         * 
         * @return area
         */
        virtual double area() const;

        /**
         * Compute the perimeter using @f$ 2 \pi r @f$
         * 
         * @return perimeter
         */
        virtual double perimeter() const;

        /**
         * Return a new duplicate Circle
         */
        virtual Shape* clone() const;

        /**
         * Print the Circle
         * 
         * @param outs the output stream--i.e., destination
         */ 
        virtual void display( std::ostream &outs ) const;

        /**
         * Read the Circle
         *
         * @param ins the input stream--i.e., source
         */
        virtual void read(std::istream &ins);
};

/**
 *  
 */
inline
double Circle::radius() const
{
    return _radius;
}

/**
 * 
 */
inline 
void Circle::radius( double r )
{
    _radius = r;
}

/**
 * 
 */
inline 
double Circle::diameter() const
{
    return (
        2 * _radius
    );
}

/**
 * 
 */
inline 
double Circle::area() const
{
    return (
        PI * _radius * _radius
    );
}

/**
 * 
 */
inline  
double Circle::perimeter() const
{
    return (
        TAU * _radius
    );
}

/**
 *  
 */
inline 
Shape* Circle::clone() const
{
    return new Circle( *this );
}

#endif