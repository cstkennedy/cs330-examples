#ifndef SPHERE_H_INCLUDED
#define SPHERE_H_INCLUDED

#include "Polyhedron.h"

/**
 * Sphere
 */
class Sphere : public Polyhedron {
    private:
        double radius;

    public:
        /**
         * Default Constructor
         */
        Sphere();

        /**
         * Construct a sphere from a provided radius
         */
        Sphere(double r);

        /**
         * Destructor
         */
        virtual ~Sphere();

        /**
         * Retrieve the radius
         */
        double getRadius() const;

        /**
         * Update the radius
         */
        void setRadius(double r);

        /**
         * Compute and return the diameter
         */
        double getDiameter() const;

        virtual Polyhedron* clone() const;
        virtual void read(std::istream& ins);
        virtual void display(std::ostream& outs) const;
        virtual void scale(double scalingFactor);
};

/**
 *
 */
inline
double Sphere::getRadius() const
{
    return this->radius;
}

/**
 *
 */
inline
void Sphere::setRadius(double r)
{
    radius = r;

    double d = getDiameter();
    boundingBox.setUpperRightVertex(d, d, d);
}

/**
 *
 */
inline
double Sphere::getDiameter() const
{
    return (2 * radius);
}

/**
 *
 */
inline Polyhedron* Sphere::clone() const
{
    return new Sphere(*this);
}

#endif