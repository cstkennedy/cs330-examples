#include "BoundingBox.h"

/**
 *
 */
BoundingBox::BoundingBox()
    :lowerLeftVertex (0, 0, 0),
     upperRightVertex(0, 0, 0)
{
}

/**
 *
 */
BoundingBox::BoundingBox(Point lowerLeft, Point upperRight)
    :lowerLeftVertex (lowerLeft),
     upperRightVertex(upperRight)
{
}

/**
 *
 */
Point BoundingBox::getUpperRightVertex() const
{
    return upperRightVertex;
}

/**
 *
 */
void BoundingBox::setUpperRightVertex(Point u)
{
    upperRightVertex = u;
}

/**
 *
 */
void BoundingBox::setUpperRightVertex(double x, double y, double z)
{
    upperRightVertex.x = x;
    upperRightVertex.y = y;
    upperRightVertex.z = z;
}

/**
 *
 */
void BoundingBox::merge(const BoundingBox& other)
{
    upperRightVertex.x = std::max(this->upperRightVertex.x,
                                  other.upperRightVertex.x);

    upperRightVertex.y = std::max(this->upperRightVertex.y,
                                  other.upperRightVertex.y);

    upperRightVertex.z = std::max(this->upperRightVertex.z,
                                  other.upperRightVertex.z);
}