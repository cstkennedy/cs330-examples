// Thomas Kennedy

package edu.odu.cs.cs330.examples.shapes;

import java.util.Scanner;

/**
 * Shape in a 2-D Cartesian Plane.
 *
 * @author Thomas J Kennedy
 */
public abstract class Shape implements Cloneable {

    /**
     * Base for Shape formatting strings.
     */
    protected static final String FMT_BASE = "%-12s: %24";

    /**
     * The string-value format string.
     */
    protected static final String FMT_STR = FMT_BASE + "s%n";

    /**
     * The double-value format string.
     */
    protected static final String FMT_DBL = FMT_BASE + ".4f%n";

    /**
     * Return the name.
     *
     * @return shape name
     */
    public abstract String name();

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

    /**
     * Read the shape.
     *
     * @param scanner the input stream--scanner in this example
     *
     * @deprecated {@link #numDims() numDims} and
     * {@link #createFromDims(double[]) createFromDims} should be used instead
     * of read.
     */
    @Deprecated
    public abstract void read(Scanner scanner);

    /**
     * The number of dimensions (double values) that need to be *read*.
     *
     * @return number of required double values
     */
    public abstract int numDims();

    /**
     * Update dimensions based on an array of double values.
     *
     * @param dims double values that represent dimensions.
     */
    public abstract void createFromDims(double[] dims);

    @Override
    public String toString()
    {
        return String.format(FMT_STR, "Name", this.name());
    }
}


