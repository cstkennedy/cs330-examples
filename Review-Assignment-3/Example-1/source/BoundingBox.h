#ifndef BOUNDINGBOX_H_INCLUDED
#define BOUNDINGBOX_H_INCLUDED

#include "Point.h"

/**
 * Rectangular prism representing the boundaries
 * x, y, and z of a polyhedron
 */
class BoundingBox {
    private:
        /**
         * Lower boundary. In this exercise, it is 
         * fixed at (0,0,0)
         */
        Point lowerLeftVertex;

        /**
         * Upper boundary
         */
        Point upperRightVertex;

    public:
        /**
         * Default Constructor
         */
        BoundingBox();

        /**
         * Construct a bounding box from lower and upper
         * points that define it
         */
        BoundingBox(Point lowerLeft, Point upperRight);

        /**
         * Retrieve the upper boundary
         */
        Point getUpperRightVertex() const;

        /**
         * Set the upper boundary using a Point
         */
        void setUpperRightVertex(Point u);

        /**
         * Set the upper boundary using the x, y, and z
         * components
         */
        void setUpperRightVertex(double x, double y, double z);

        /**
         * Merge two bounding boxes, taking the
         * largest values for each of x, y, and z
         */
        void merge(const BoundingBox& other);

        /**
         * Apply a scaling factor
         */
        void scale(double s);
};

/**
 *
 */
inline
void BoundingBox::scale(double s)
{
    upperRightVertex.scale(s);
}

#endif