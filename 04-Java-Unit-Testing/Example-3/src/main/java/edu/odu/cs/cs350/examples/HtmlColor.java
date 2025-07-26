package edu.odu.cs.cs350.examples;

import java.util.Objects;

/**
 * An HTML Color is represented by a combination of red, green and blue. Each
 * component (red, green, and blue) must fall in the range [0, 255].
 *
 * The methods in this class clamp all values to the range [0, 255].
 */
public class HtmlColor implements Cloneable
{
    /**
     * Clamp is an implementation of the mathematical operation that takes a
     * value and ensures is falls within a specified range.
     *
     *   - Any value that is less than 0 will be replaced with zero.
     *   - Any value that is greater than 255 will be replaced with 255.
     *
     * @param val raw value
     *
     * @return clamped value
     */
    public static int clamp(int val)
    {
        return Math.min(Math.max(0, val), 255);
    }

    private int red;   ///< red color component [0,255]
    private int green; ///< green color component [0,255]
    private int blue;  ///< blue color component [0,255]

    /**
     * Construct an HTML Color with all
     * attributes set to 0 (i.e., black, #000000)
     */
    public HtmlColor()
    {
        this(0, 0, 0);
    }

    /**
     * Construct an HTML Color
     *
     * @param rVal red value
     * @param gVal green value
     * @param bVal blue value
     *
     * All values are clamped to the range [0, 255]
     */
    public HtmlColor(int rVal, int gVal, int bVal)
    {
        this.setRed(rVal);
        this.setGreen(gVal);
        this.setBlue(bVal);
    }

    /**
     * Retrieve the red component
     */
    public int getRed()
    {
        return this.red;
    }

    /**
     * Set the red component
     *
     * @param val new value
     */
    public void setRed(int val)
    {
        this.red = clamp(val);
    }

    /**
     * Retrieve the green component
     */
    public int getGreen()
    {
        return this.green;
    }

    /**
     * Set the green component
     *
     * @param val new value
     */
    public void setGreen(int val)
    {
        this.green = clamp(val);
    }

    /**
     * Retrieve the blue component
     */
    public int getBlue()
    {
        return this.blue;
    }

    /**
     * Set the blue component
     *
     * @param v new value
     */
    public void setBlue(int val)
    {
        this.blue = clamp(val);
    }

    /**
     * Two colors are considered equal if their red, green, and blue
     * components are equivalent.
     */
    public boolean equals(Object obj)
    {
        if (!(obj instanceof HtmlColor)) {
            return false;
        }

        HtmlColor rhs = (HtmlColor) obj;

        return this.red == rhs.red
            && this.green == rhs.green
            && this.blue == rhs.blue;
    }

    /**
     * Hash codes are computed using red, green, and blue
     */
    public int hashCode()
    {
        return Objects.hash(this.red, this.green, this.blue);
    }

    /**
     * Return a (deep) copy of this object.
     */
    @Override
    public Object clone()
    {
        return new HtmlColor(this.red, this.green, this.blue);
    }

    /**
     * Colors are represented in the form #RRGGBB where RR, GG, and BB are the
     * red, green and blue components, respectively, in base 16 (hexadecimal).
     */
    public String toString()
    {
        return String.format("#%02X%02X%02X", red, green, blue).toUpperCase();
    }

}
