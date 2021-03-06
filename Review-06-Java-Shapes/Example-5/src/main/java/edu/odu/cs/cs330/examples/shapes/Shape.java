// Thomas Kennedy
// CS 330 Fall 2018

package edu.odu.cs.cs330.examples.shapes;

import java.util.Scanner;

/**
 * Shape in a 2-D Cartesian Plane.
 *
 * @author Thomas J Kennedy
 */
public abstract class Shape implements Cloneable {

    /**
     * Label Output Width.
     */
    protected static final int WIDTH_LABEL = 12;

    /**
     * Value Output Width.
     */
    protected static final int WIDTH_VALUE = 24;

    /**
     * The string-value format string.
     */
    protected static final String FMT_STR = getFormat("s%n");

    /**
     * The double-value format string.
     */
    protected static final String FMT_DBL = getFormat(".4f%n");

    /**
     * Generate the format string for a label-value pair.
     *
     * @param value_format trailing portion of a format String
     * @return complete label-value format String
     */
    protected static String getFormat(String value_format)
    {
        return "%-" + WIDTH_LABEL + "s: %" + WIDTH_VALUE + value_format;
    }

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
        return String.format(FMT_STR, "Name", this._name);
    }
}


