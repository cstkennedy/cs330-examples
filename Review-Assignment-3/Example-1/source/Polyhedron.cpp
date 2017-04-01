#include "Polyhedron.h"
#include "PolyhedronFactory.h"

/**
 *
 */
Polyhedron::Polyhedron()
    :type("Polyhedron"),
     boundingBox()
{
}

/**
 *
 */
Polyhedron::Polyhedron(const char* t)
    :type(t),
     boundingBox()
 {
 }

/**
 *
 */
Polyhedron::~Polyhedron()
{
    // No dynamic memory
}

/**
 *
 */
void Polyhedron::display(std::ostream& outs) const
{
    outs << "[" << type << "] " 
         << boundingBox.getUpperRightVertex() 
         << "->"; 
}

/**
 *
 */
void Polyhedron::scale(double scalingFactor)
{
    boundingBox.scale(scalingFactor);
}

/**
 *
 */
std::istream& operator>>(std::istream& ins, Polyhedron*& ply)
{
    std::string polyhedronType;

    if (ins >> polyhedronType) {
        ply = PolyhedronFactory::createPolyhedron(polyhedronType);

        if (ply != nullptr) {
            ply->read(ins);
        }
        else {
            getline(ins, polyhedronType);
        }
    }

    return ins;
}
