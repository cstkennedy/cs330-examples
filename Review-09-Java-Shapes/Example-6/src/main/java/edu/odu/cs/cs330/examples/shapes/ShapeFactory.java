// Thomas Kennedy
// CS 330 Fall 2020

package edu.odu.cs.cs330.examples.shapes;

/**
 * The Shape Creating Wizard.
 *
 * @author Thomas J Kennedy
 */
public class ShapeFactory {
    /**
     * Listing of known shapes.
     */
    private static final Shape[] _KNOWN_SHAPES = new Shape[] {
        new Triangle(),
        new RightTriangle(),
        new EquilateralTriangle(),
        new Square(),
        new Circle()
    };

    /**
     *  Create a Shape.
     *
     *  @param name the shape to be created
     *
     *  @return A shape with the specified name
     *      or null if no matching shape is found
     *
     * @throws CloneNotSupportedException if a model shape is found and the
     *     copy (i.e., clone) operation fails
     */
    public static Shape createShape(String name)
        throws CloneNotSupportedException
    {
        for (Shape shape : _KNOWN_SHAPES) {
            if (shape.name().equals(name)) {
                return (Shape) shape.clone();
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
        for (Shape shape : _KNOWN_SHAPES) {
            if (shape.name().equals(name)) {
                return true;
            }
        }

        return false;
    }

    /**
     *  Print a list of known Shapes.
     *
     * @return a string containing a newline delimited list of known shape names
     */
    public static String listKnown()
    {
        StringBuilder bld = new StringBuilder();

        for (Shape shape : _KNOWN_SHAPES) {
            bld.append(shape.name() + "\n");
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
        return _KNOWN_SHAPES.length;
    }
}
