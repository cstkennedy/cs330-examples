#ifndef POLYHEDRON_H_INCLUDED
#define POLYHEDRON_H_INCLUDED

#include <cmath>
#include <cstdlib>
#include <iostream>
#include <string>

#include "Point.h"
#include "BoundingBox.h"

/**
 * Abstract Polyhedron Base Class
 */
class Polyhedron {
    private:        
        /**
         * A string representing the name of this polyhedron
         */
        std::string type;

    protected:
        /**
         * Box (rectangular prism) that contains this element
         */
        BoundingBox boundingBox;

    public:

        /**
         * Default Constructor
         */
        Polyhedron();

        /**
         * Constructor which allows
         * a name to be set
         *
         * @param t c-string representing the polyhedron name
         */
        Polyhedron(const char* t);

        /**
         * Destructor
         */
        virtual ~Polyhedron();

        /**
         * Get the polyhedron name
         */
        std::string getType() const;

        /**
         * set the polyhedron name
         */
        void setType(std::string t);

        /**
         * Retrieve the bounding box
         */
        BoundingBox getBoundingBox() const;

        /**
         * Duplicate the polyhedron
         */
        virtual Polyhedron* clone() const = 0;

        /**
         * Retrieve and reconstruct the polyhedron
         * from an input stream
         */
        virtual void read(std::istream& ins) = 0;

        /**
         * Print the polyhedron
         */
        virtual void display(std::ostream& outs) const;

        /**
         * Check if two polyhedra have matching types
         */
        bool isTypeMatch(const Polyhedron* rhs) const;

        /**
         * Apply a geometric scaling operation
         */
        virtual void scale(double scalingFactor);
};

/**
 *
 */
inline
std::string Polyhedron::getType() const
{
    return type;
}

/**
 *
 */
inline
void Polyhedron::setType(std::string t)
{
    type = t;
}

/**
 *
 */
inline
BoundingBox Polyhedron::getBoundingBox() const
{
    return boundingBox;
}

/**
 *
 */
inline
bool Polyhedron::isTypeMatch(const Polyhedron* rhs) const
{
    return this->getType() == rhs->getType();
}

/**
 * Stream insertion operator (inline wrapper for display)
 */
inline
std::ostream& operator<<(std::ostream& outs, const Polyhedron& ply) {
    ply.display(outs);

    return outs;
}

/**
 * Stream extraction (input) operator
 */
std::istream& operator>>(std::istream& ins, Polyhedron*& ply);


#endif