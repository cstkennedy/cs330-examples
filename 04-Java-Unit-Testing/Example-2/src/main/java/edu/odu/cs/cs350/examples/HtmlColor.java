package edu.odu.cs.cs350.examples;

import java.util.Objects;

/**
 * An HTML Color is represented by a combination of red, green and blue. Each
 * component (red, green, and blue) must fall in the range [0, 255].
 */
public class HtmlColor implements Cloneable
{
    /**
     * A situation occurred where the red component, green component, or blue
     * component (or some combination) of the three (3) fell outside the range
     * [0, 255].
     */
    public static class InvalidColorException extends Exception
    {
        public InvalidColorException(String message)
        {
            super(message);
        }
    }

    private int red;   ///< red color component [0,255]
    private int green; ///< green color component [0,255]
    private int blue;  ///< blue color component [0,255]

    /**
     * Construct an HTML Color with all
     * attributes set to 0 (i.e., black, #000000)
     */
    public HtmlColor()
        throws InvalidColorException
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
     * @throws HtmlColor.InvalidColorException if any of rVal, gVal, or bVal
     * are outside the range [0, 255]
     */
    public HtmlColor(int rVal, int gVal, int bVal)
        throws InvalidColorException
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
     *
     * @throws HtmlColor.InvalidColorException if val is outside the range [0, 255]
     */
    public void setRed(int val)
        throws InvalidColorException
    {
        if (val < 0 || val > 255) {
            throw new InvalidColorException(
                "Red component must be in the range [0, 255]"
            );
        }

        this.red = val;
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
     *
     * @throws HtmlColor.InvalidColorException if val is outside the range [0, 255]
     */
    public void setGreen(int val)
        throws InvalidColorException
    {
        if (val < 0 || val > 255) {
            throw new InvalidColorException(
                "Green component must be in the range [0, 255]"
            );
        }

        this.green = val;
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
     *
     * @throws HtmlColor.InvalidColorException if val is outside the range [0, 255]
     */
    public void setBlue(int val)
        throws InvalidColorException
    {
        if (val < 0 || val > 255) {
            throw new InvalidColorException(
                "Blue component must be in the range [0, 255]"
            );
        }

        this.blue = val;
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
        HtmlColor cpy = null;

        try {
            cpy = new HtmlColor(this.red, this.green, this.blue);
        }
        catch (InvalidColorException _ignore) {
            // It is impossible to create an InvalidHtmlColor from a valid one.
        }

        return cpy;
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
