#include <utility>
#include <algorithm>

#include "ShapeCollection.h"

/**
 * 
 */
ShapeCollection::ShapeCollection()
    :shapes() // Is this necessary?
{
}

/**
 * 
 */
ShapeCollection::ShapeCollection(int n)
{       
    shapes.reserve(n);
}

/**
 * 
 */
ShapeCollection::ShapeCollection(const ShapeCollection& src)
{
    for (const Shape* s: src.shapes) {
        shapes.push_back(s->clone());
    }
}

/**
 * 
 */
ShapeCollection::~ShapeCollection()
{
    for (Shape* s : this->shapes) {
        delete s;
    }
}

/**
 *
 */
bool ShapeCollection::addShape(Shape* toAdd)
{
    if (toAdd == nullptr) {
        return false;
    }

    shapes.push_back(toAdd);
    return true;
}

/**
 * 
 */
ShapeCollection& ShapeCollection::operator=(ShapeCollection rhs)
{    
    std::swap(*this, rhs);

    return *this;
}

/**
 * 
 */
void ShapeCollection::display(std::ostream& outs) const
{
    for (const Shape* s : this->shapes) {
        outs << *s << "\n";
    }
}


void swap(ShapeCollection& lhs, ShapeCollection& rhs)
{
    using std::swap;

    swap(lhs.shapes, rhs.shapes);
}