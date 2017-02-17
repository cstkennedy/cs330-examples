// Thomas Kennedy
// CS 330 Fall 2014

#ifndef SHAPEFACTORY_H_INCLUDED
#define SHAPEFACTORY_H_INCLUDED

#include <iostream>

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
         * Name Shape Pair 2-tuple( name, model )
         */
        struct ShapePair{            
            std::string _name;   ///< Name of the shape to clone
            Shape      *_model;  ///< Model of the shape to clone

            /**
             * Default Constructor - Used as sentinel
             */
            ShapePair();

            /**
             * Non-Default Constructor
             *
             * @param name the name of a shape
             * @param shape a cloneable shape
             */
            ShapePair( std::string name, Shape *shape );

            /**
             * Deconstruct a ShapePair
             */
            ~ShapePair();
        };

        static ShapePair _known_shapes[];  ///< Listing of known shapes

    public:
        /**
         * Create a Shape
         *
         * @param name the shape to be created
         *
         * @return A shape with the specified name
         *     or nullptr if no matching shape is found
         */
        static Shape* createShape( std::string name );

        /**
         * Determine whether a given shape is known
         *
         * @param name the shape for which to query
         */
        static bool isKnown( std::string name );

        /**
         * Print a list of known Shapes
         *
         * @param outs the output stream
         */
        static void listKnown( std::ostream &outs );

        /**
         * Determine the number of known Shapes
         *
         * @return the number of known shapes
         *
         */
        static int numberKnown();
};

#endif