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
     * Geometric Name of the 2-D Figure.
     */
    protected String _name;

    /**
     * Default Shape Constructor.
     */
    public Shape()
    {
        this._name = "Shape";
    }

    /**
     * Shape Constructor.
     *
     * @param name the desired Shape name
     */
    public Shape(String name)
    {
        this._name = name;
    }

    /**
     * Return the name.
     *
     * @return shape name
     */
    public String name()
    {
        return this._name;
    }

    /**
     * Modify the name.
     *
     * @param _name new Shape name
     * @return shape name
     */
    protected void name(String _name)
    {
        this._name = _name;
    }

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
     */
    public abstract void read(Scanner scanner);

    @Override
    public String toString()
    {
        return String.format(FMT_STR, "Name", this.name());
    }
}


