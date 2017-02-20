#ifndef __SHAPECOLLECTION_H__
#define __SHAPECOLLECTION_H__

#include <iostream>
#include <vector>

#include "shape.h"

/**
 * A container for all objects that
 * implement the _Shape_ interface 
 */
class ShapeCollection {
    public:
        // Iterators (non-const and const)
        typedef std::vector<Shape*>::iterator iterator;
        typedef std::vector<Shape*>::iterator const_iterator;

    private:
        /**
         * Container of Shape pointers (`Shape*`)
         */
        std::vector<Shape*> shapes;

    public:
        /**
         * Default Shape Constructor
         */
        ShapeCollection();

        /**
         * Construct a Shape Collection and
         * `reserve` space
         *
         * @param n number of `Shape*` blocks
         * to reserve
         */
        ShapeCollection(int n);

        /**
         * Copy Constructor
         * <p>
         * Each `Shape*` must be explicitly copied even
         * though they are in a `std::vector`
         * <p>
         * Shallow-copy vs. Deep-copy 
         */
        ShapeCollection(const ShapeCollection& src);

        /**
         * Destructor
         * <p>
         * Each `Shape*` must be deleted even
         * though they are in a `std::vector`
         */
        ~ShapeCollection();

        /**
         * Return the number of `Shape*`s
         * in the `ShapeCollection`
         */
        size_t size() const;

        /**
         * Add a `Shape` to the collection
         *
         * @param toAdd a pointer to the `Shape`
         * object to add
         *
         * @pre `toAdd != nullptr`
         */
        bool addShape(Shape* toAdd);

        /**
         * Assignment Operator
         * <p>
         * Let us use the _copy and swap_ idiom
         * <p>
         * **Note** that this is using **Pass-by-Value**
         * not **Pass-by-Const-Reference**
         */
        ShapeCollection& operator=(ShapeCollection rhs);

        /**
         * Print all Shape Objects in the `ShapeCollection` 
         */
        void display(std::ostream& outs) const;

        /**
         * Swap the contents of two `ShapeCollection`s
         * <p>
         * I am using a friend function here and only here (under protest)
         * <p>
         * [Refer here](http://stackoverflow.com/questions/3279543/what-is-the-copy-and-swap-idiom)
         */
        friend 
        void swap(ShapeCollection& lhs, ShapeCollection& rhs);
};

/**
 *
 */
inline
size_t ShapeCollection::size() const
{
    return shapes.size();
}

/**
 * Stream Insertion (Colloquially the Output/cout Operator)
 */
inline
std::ostream& operator<<(std::ostream& outs, const ShapeCollection& prt)
{
    prt.display(outs);

    return outs;
}

#endif