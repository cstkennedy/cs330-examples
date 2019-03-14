// Thomas Kennedy
// CS 330 Fall 2014

package shapes;

/**
 * Shape in a 2-D Cartesian Plane
 */
public abstract class Shape implements Cloneable
{

    protected static final int WIDTH_LABEL = 12; ///< Label Output Width
    protected static final int WIDTH_VALUE = 24; ///< Value Output Width

    /**
     * Format String for a label and Floating Point value
     */
    protected static final String FPT_FMT = "%-" + WIDTH_LABEL + "s: "
                                          + "%" + WIDTH_VALUE + ".4f%n";

    /**
     * Format String for a label and String value
     */
    protected static final String STR_FMT = "%-" + WIDTH_LABEL + "s: "
                                          + "%" + WIDTH_VALUE + "s%n";

    /**
     * Generate the format string for a label-value pair
     *
     * @param value_format trailing portion of a format String
     * @return complete label-value format String
     */
    protected static String getFormat(String value_format)
    {
        return "%-" + WIDTH_LABEL +"s: %" + WIDTH_VALUE + value_format;
    }

    protected String _name; ///< Geometric Name of the 2-D Figure

    /**
     * Default Shape Constructor
     */
    public Shape()
    {
        this._name = "Shape";
    }

    /**
     * Shape Constructor
     *
     * @param name the desired Shape name
     */
    public Shape(String name)
    {
        this._name = name;
    }

    /**
     * Return the name
     */
    public String name()
    {
        return this._name;
    }

    /**
     * Modify the name
     *
     * @param _name new Shape name
     * @return shape name
     */
    protected void name(String _name)
    {
        this._name = _name;
    }

    /**
     * Compute the area
     *
     * @return area
     */
    public abstract double area();

    /**
     * Compute the perimeter
     *
     * @return perimeter
     */
    public abstract double perimeter();

    /**
     * Return a new duplicate Shape
     */
    public abstract Shape clone();

    /**
     * Print the shape
     */
    @Override
    public String toString()
    {
        return String.format(STR_FMT, "Name", this._name);

        //return "Name: " + this_.name;
    }
}


