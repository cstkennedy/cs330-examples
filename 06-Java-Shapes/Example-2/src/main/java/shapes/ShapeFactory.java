// Thomas Kennedy

package shapes;

import java.lang.StringBuilder;

/**
 * The Shape Creating Wizard.
 *
 * @author Thomas J Kennedy
 */
public class ShapeFactory
{
    /**
     * Name Shape Pair 2-tuple(name, model)
     */
    private static class ShapePair
    {
        public String _name;  ///< Name of the shape to clone
        public Shape  _model; ///< Model of the shape to clone

        /**
         * Default Constructor - Used as sentinel
         */
        public ShapePair()
        {
            this._name  = new String();
            this._model = null;
        }

        /**
         * Non-Default Constructor
         *
         * @param name the name of a shape
         * @param shape a cloneable shape
         */
        public ShapePair(String name, Shape shape)
        {
            this._name  = name;
            this._model = shape;
        }

        /**
         * Print the ShapePair
         */
        public String toString()
        {
            return "  " + this._name + "\n";
        }
    }

    private static ShapePair[] _known_shapes = new ShapePair[] {
        new ShapePair("Triangle",             new Triangle()           ),
        new ShapePair("Right Triangle",       new RightTriangle()      ),
        new ShapePair("Equilateral Triangle", new EquilateralTriangle()),
        new ShapePair("Square",               new Square()             ),
        new ShapePair("Circle",               new Circle()             )
    }; ///< Listing of known shapes

    /**
     * Create a Shape.
     *
     * @param name the shape to be created
     *
     * @return A shape with the specified name
     *     or null if no matching shape is found
     *
     * @throws CloneNotSupportedException if a model shape is found and the
     *     copy (i.e., clone) operation fails
     */
    public static Shape createShape(String name)
    {
        for (ShapePair pair : _known_shapes) {
            if ((pair._name).equals(name)) {
                return (pair._model).clone();
            }
        }

        return null;
    }

    /**
     * Determine whether a given shape is known.
     *
     * @param name the shape for which to query
     *
     * @return true if the specified name corresponds to a known shape
     */
    public static boolean isKnown(String name)
    {
        for (ShapePair pair : _known_shapes) {
            if ((pair._name).equals(name)) {
                return true;
            }
        }

        return false;
    }

    /**
     * Print a list of known Shapes.
     *
     * @return a string containing a newline delimited list of know shape names
     */
    public static String listKnown()
    {
        StringBuilder bld = new StringBuilder();

        for (ShapePair pair : _known_shapes) {
            bld.append(pair);  // Note: Implicit call to pair.toString()
        }

        return bld.toString();
    }

    /**
     *  Determine the number of known Shapes.
     *
     *  @return the number of known shapes
     */
    public static int numberKnown()
    {
        return _known_shapes.length;
    }
}
