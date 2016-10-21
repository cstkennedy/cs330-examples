// Thomas Kennedy
// CS 330 Fall 2014

#ifndef SHAPE_H_INCLUDED
#define SHAPE_H_INCLUDED

#include <iostream>
#include <string>

/**
 * Shape in a 2-D Cartesian Plane
 */
class Shape{
    protected:
        static const int WIDTH_LABEL;  ///< Label Output Width
        static const int WIDTH_VALUE;  ///< Value Output Width

        std::string _name;             ///< Geometric Name of the 2-D Figure

        /**
         * Modify the name
         *
         * @param _name new Shape name
         * @return shape name
         */
        void name( std::string _name );

    public:
        /**
         * Default Shape Constructor
         */
        Shape();

        /**
         * Shape Constructor
         * 
         * @param name the desired Shape name
         */
        Shape( std::string name );

        /**
         * Shape Destructor
         */
        virtual ~Shape();

        /**
         * Return the name
         */
        std::string name() const;

        /**
         * Compute the area
         *
         * @return area
         */
        virtual double area() const = 0;

        /**
         * Compute the perimeter
         *
         * @return perimeter
         */
        virtual double perimeter() const = 0;

        /**
         * Return a new duplicate Shape
         */
        virtual Shape* clone() const = 0;

        /**
         * Print the shape
         *
         * @param outs the output stream--i.e., destination
         */
        virtual void display( std::ostream &outs ) const;
};

/**
 *
 */
inline
std::string Shape::name() const
{
    return this->_name;
}

/**
 *
 */
inline
void Shape::name( std::string _name )
{
    this->_name = _name;
}

/**
 * Overloaded Stream Insertion Operator
 *
 * @param outs the output stream
 * @param prt the Shape to print
 *
 * @return the modified output stream
 */
inline std::ostream& operator<< ( std::ostream& outs, const Shape &prt )
{
    prt.display( outs );

    return outs;
}

#endif