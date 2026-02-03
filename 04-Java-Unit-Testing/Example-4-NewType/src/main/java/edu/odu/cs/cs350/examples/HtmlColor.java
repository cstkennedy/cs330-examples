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
     * Represents a color component (red, green, or blue).
     */
    public static class Component
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

        // Not implemented
        public static Component from(int raw_val)
        {
            return null;
        }

        public static Component from_clamped(int raw_val)
        {
            final int clamped_val = Component.clamp(raw_val);

            return new Component(clamped_val);
        }

        private int val;

        private Component()
        {
            this.val = 0;
        }

        private Component(int sanitized_val)
        {
            this.val = sanitized_val;
        }

        public int value()
        {
            return this.val;
        }

        @Override
        public boolean equals(Object rhsObj)
        {
            if (!(rhsObj instanceof Component)) {
                return false;
            }

            Component rhs = (Component) rhsObj;

            return this.val == rhs.val;
        }

        @Override
        public int hashCode()
        {
            return this.val;
        }
    }

    private Component red;   ///< red color component [0,255]
    private Component green; ///< green color component [0,255]
    private Component blue;  ///< blue color component [0,255]

    /**
     * Construct an HTML Color with all
     * attributes set to 0 (i.e., black, #000000)
     */
    public HtmlColor()
    {
        this.red = Component.from_clamped(0);
        this.green = Component.from_clamped(0);
        this.blue = Component.from_clamped(0);
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
    public HtmlColor(Component rVal, Component gVal, Component bVal)
    {
        this.red = rVal;
        this.green = gVal;
        this.blue = bVal;
    }

    /**
     * Retrieve the red component
     */
    public Component getRed()
    {
        return this.red;
    }

    /**
     * Set the red component
     *
     * @param val new value
     */
    public void setRed(Component val)
    {
        this.red = val;
    }

    /**
     * Retrieve the green component
     */
    public Component getGreen()
    {
        return this.green;
    }

    /**
     * Set the green component
     *
     * @param val new value
     */
    public void setGreen(Component val)
    {
        this.green = val;
    }

    /**
     * Retrieve the blue component
     */
    public Component getBlue()
    {
        return this.blue;
    }

    /**
     * Set the blue component
     *
     * @param v new value
     */
    public void setBlue(Component val)
    {
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

        return this.red.equals(rhs.red)
            && this.green.equals(rhs.green)
            && this.blue.equals(rhs.blue);
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
        return String.format(
            "#%02X%02X%02X",
            red.value(),
            green.value(),
            blue.value()
        ).toUpperCase();
    }

}
