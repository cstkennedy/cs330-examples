// Thomas Kennedy
// CS 330 Fall 2019

#ifndef SHAPEFACTORY_H_INCLUDED
#define SHAPEFACTORY_H_INCLUDED

#include <iostream>
#include <memory>
#include <utility>
#include <string>
#include <string_view>

#include "shape.h"
#include "triangle.h"
#include "rightTriangle.h"
#include "equilateralTriangle.h"

#include "square.h"
#include "circle.h"

/**
 * The Shape Creating Wizard
 */
class ShapeFactory{
    private:
        /**
         * Name Shape Pair 2-tuple(name, model)
         *
         * No more workarounds required!
         */
        using ShapePair = std::pair<std::string_view, std::unique_ptr<Shape>>;

        static ShapePair _KNOWN_SHAPES[];  ///< Listing of known shapes

    public:
        /**
         * Create a Shape
         *
         * @param name the shape to be created
         *
         * @return A shape with the specified name
         *     or nullptr if no matching shape is found
         */
        static Shape* createShape(std::string_view name);

        /**
         * Determine whether a given shape is known
         *
         * @param name the shape for which to query
         */
        static bool isKnown(std::string_view name);

        /**
         * Print a list of known Shapes
         *
         * @param outs the output stream
         */
        static void listKnown(std::ostream &outs);

        /**
         * Determine the number of known Shapes
         *
         * @return the number of known shapes
         *
         */
        static int numberKnown();
};

/**
 * Create the appropriate Shape class
 *
 * How is **inheritance** used?
 */
std::istream& operator>>(std::istream& ins, Shape*& rd);

#endif
