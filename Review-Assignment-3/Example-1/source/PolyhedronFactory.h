#ifndef POLYHEDRONFACTORY_H_INCLUDED
#define POLYHEDRONFACTORY_H_INCLUDED

#include <iostream>

class Polyhedron;

/**
 * The Polyhedron Creating Wizard
 */
class PolyhedronFactory {
    private:
        /**
         * Name Polyhedron Pair 2-tuple( name, model )
         * <p>
         * This should really have the Big-3 implemented.
         * However, I worked around this by using reference
         * variables in my for-each (range-based) for loops.
         * <p>
         * Such workarounds can, but should not, be done.
         * <p>
         * In this case, it serves as an academic discussion.
         */
        struct PolyhedronPair {            
            std::string   _name;   ///< Name of the polyhedron to clone
            Polyhedron*   _model;  ///< Model of the polyhedron to clone

            /**
             * Default Constructor - Used as sentinel
             */
            PolyhedronPair();

            /**
             * Non-Default Constructor
             *
             * @param name the name of a polyhedron
             * @param polyhedron a cloneable polyhedron
             */
            PolyhedronPair(std::string name, Polyhedron* polyhedron);

            /**
             * Deconstruct a PolyhedronPair
             */
            ~PolyhedronPair();
        };

        static PolyhedronPair _known_polyhedra[];  ///< Listing of known polyhedra

    public:
        /**
         * Create a Polyhedron
         *
         * @param name the polyhedron to be created
         *
         * @return A polyhedron with the specified name
         *     or nullptr if no matching polyhedron is found
         */
        static Polyhedron* createPolyhedron(std::string name);

        /**
         * Determine whether a given polyhedron is known
         *
         * @param name the polyhedron for which to query
         */
        static bool isKnown(std::string name);

        /**
         * Print a list of known Polyhedrons
         *
         * @param outs the output stream
         */
        static void listKnown(std::ostream& outs);

        /**
         * Determine the number of known Polyhedrons
         *
         * @return the number of known polyhedrons
         *
         */
        static int numberKnown();
};

#endif