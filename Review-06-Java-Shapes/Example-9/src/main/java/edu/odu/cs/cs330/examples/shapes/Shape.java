// Thomas Kennedy

package edu.odu.cs.cs330.examples.shapes;

/**
 * Shape in a 2-D Cartesian Plane.
 *
 * @author Thomas J Kennedy
 */
public interface Shape extends Cloneable, TraitFromDimensions {

    /**
     * Base for Shape formatting strings.
     */
    public static final String FMT_BASE = "%-12s: %24";

    /**
     * The string-value format string.
     */
    public static final String FMT_STR = FMT_BASE + "s%n";

    /**
     * The double-value format string.
     */
    public static final String FMT_DBL = FMT_BASE + ".4f%n";

    /**
     * Return the name.
     *
     * @return shape name
     */
    public abstract String name();
    // TODO: remove abstract - it is left here for emphasis (i.e., discussion).
    // However, it is a code smell in interfaces.

    /**
     * Compute the area.
     *
     * @return area
     */
    public abstract double area();

    /**
     * Compute the perimeter.
     *
     * @return perimeter
     */
    public abstract double perimeter();

    /**
     * Return a new duplicate Shape.
     */
    public abstract Object clone() throws CloneNotSupportedException;
}


