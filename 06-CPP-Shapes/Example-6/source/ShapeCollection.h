#ifndef __SHAPECOLLECTION_H__
#define __SHAPECOLLECTION_H__

#include <iostream>
#include <vector>
#include <utility>

#include "shape.h"

/**
 * A container for all objects that
 * implement the _Shape_ interface
 */
class ShapeCollection {
    public:
        // Iterators (non-const and const)
        using iterator = std::vector<Shape*>::iterator;
        using const_iterator = std::vector<Shape*>::const_iterator;

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
         * Allow access to the _beginning_ of the
         * `ShapeCollection` via an
         * iterator.
         */
        iterator begin();

        /**
         * Allow access to the _beginning_ of the
         * `ShapeCollection` via an
         * const_iterator.
         */
        const_iterator begin() const;

        /**
         * Allow access to the _end_ of the
         * `ShapeCollection` via an
         * iterator.
         */
        iterator end();

        /**
         * Allow access to the _end_ of the
         * `ShapeCollection` via a
         * const_iterator.
         */
        const_iterator end() const;

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
         * Swap the contents of two `ShapeCollection`s
         * <p>
         * I am using a friend function here and only here (under protest)
         * <p>
         * [Refer here](http://stackoverflow.com/questions/3279543/what-is-the-copy-and-swap-idiom)
         */
        friend
        void swap(ShapeCollection& lhs, ShapeCollection& rhs);
};

//------------------------------------------------------------------------------
inline
size_t ShapeCollection::size() const
{
    return shapes.size();
}

#endif
