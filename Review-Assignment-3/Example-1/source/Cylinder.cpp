#include "Cylinder.h"

/**
 * 
 */
Cylinder::Cylinder()
    :Polyhedron("Cylinder"),
     height(0),
     radius(0)
{
}

/**
 * 
 */
Cylinder::Cylinder(double r, double h)
    :Polyhedron("Cylinder"),
     height(h),
     radius(r)
{
    double d = this->getDiameter();
    boundingBox.setUpperRightVertex(d, d, height);
}

/**
 * 
 */
Cylinder::~Cylinder()
{
}

/**
 * 
 */
void Cylinder::read(std::istream& ins)
{
    ins >> height >> radius;

    double d = this->getDiameter();
    boundingBox.setUpperRightVertex(d, d, height);
}

/**
 * 
 */
void Cylinder::display(std::ostream& outs) const
{
    Polyhedron::display(outs);

    outs << "Radius: " << radius 
         << " "
         << "Height: " << height;
}

/**
 * 
 */
void Cylinder::scale(double scalingFactor)
{
    radius *= scalingFactor;
    height *= scalingFactor;

    Polyhedron::scale(scalingFactor);
}