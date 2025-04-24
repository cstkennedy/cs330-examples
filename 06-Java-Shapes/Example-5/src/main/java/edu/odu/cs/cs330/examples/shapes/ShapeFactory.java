// Thomas Kennedy

package edu.odu.cs.cs330.examples.shapes;

import java.util.Map;
import java.util.LinkedHashMap;

/**
 * The Shape Creating Wizard.
 *
 * @author Thomas J Kennedy
 */
public class ShapeFactory {
    /**
     * Listing of known shapes.
     */
    private static final Map<String, Shape> _KNOWN_SHAPES = new LinkedHashMap<>() {{
        put("Triangle", new Triangle());
        put("Right Triangle", new RightTriangle());
        put("Equilateral Triangle", new EquilateralTriangle());
        put("Square", new Square());
        put("Circle", new Circle());
    }};

    /**
     * ShapeFactory is a collection of static functions. There is no reason to
     * instantiate a ShapeFactory object.
     */
    private ShapeFactory()
    {
        // do not allow ShapeFactory to be instantiated.
    }

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
        throws CloneNotSupportedException
    {
        if (!isKnown(name)) {
            return null;
        }

        final Shape modelShape = _KNOWN_SHAPES.get(name);
        return (Shape) modelShape.clone();
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
        return _KNOWN_SHAPES.containsKey(name);
    }

    /**
     * Print a list of known Shapes.
     *
     * @return a string containing a newline delimited list of known shape names
     */
    public static String listKnown()
    {
        /*
        StringBuilder bld = new StringBuilder();

        for (String name : _KNOWN_SHAPES.keySet()) {
            bld.append(name + "\n");
        }

        return bld.toString();
        */

        return _KNOWN_SHAPES.keySet()
            .stream()
            .collect(java.util.stream.Collectors.joining("\n", "", "\n"));
    }

    /**
     *  Determine the number of known Shapes.
     *
     *  @return the number of known shapes
     */
    public static int numberKnown()
    {
        return _KNOWN_SHAPES.size();
    }
}
