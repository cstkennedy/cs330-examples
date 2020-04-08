// Thomas Kennedy
// CS 330 Fall 2020

package edu.odu.cs.cs330.examples.shapes;

import java.util.Scanner;

/**
 * A Triangle with all sides set to the same length.
 *
 * @author Thomas J Kennedy
 */
public class EquilateralTriangle extends Triangle implements Cloneable {
    /**
     * sqrt(3) / 4
     */
    private static final double ROOT_3_DIV_4 = Math.sqrt(3) / 4;

    /**
     * Construct an EquilateralTriangle
     * with all sides set to 1.
     */
    public EquilateralTriangle()
    {
        this(1);
    }

    /**
     * Construct an EquilateralTriangle.
     *
     * @param side the desired side length
     */
    public EquilateralTriangle(double side)
    {
        this.side(side);
    }

    /**
     * Construct an EquilateralTriangle.
     *
     * @param src the EquilateralTriangle to copy
     */
    public EquilateralTriangle(EquilateralTriangle src)
    {
        this.side(src.side());
    }

    /**
     * Compute the height using
     * @f$ height = \frac{5}{4}side @f$.
     *
     * @return height
     */
    public double height()
    {
        return Math.sqrt(1.25 * (side() * side()));
    }

    /**
     * Return the length of one side.
     *
     * @return the length of one side
     */
    public double side()
    {
        return _side_a;
    }

    /**
     * Modify the side length.
     *
     * @param s the desired side length
     */
    public void side(double s)
    {
        _side_a = s;
        _side_b = s;
        _side_c = s;
    }

    @Override
    public String name()
    {
        return "Equilateral Triangle";
    }

    /**
     * Compute the area using
     * @f$ Area=\frac{\sqrt{3}}{4}side^2 @f$.
     *
     * @return the area
     */
    @Override
    public double area()
    {
        return ROOT_3_DIV_4 * side() * side();
    }

    /**
     * Return a new duplicate EquilateralTriangle.
     */
    @Override
    public Object clone() throws CloneNotSupportedException
    {
        return new EquilateralTriangle(this);
    }

    /**
     * Read the shape.
     *
     * @param scanner the input stream--scanner in this example
     */
    @Override
    public void read(Scanner scanner)
    {
        this.side(scanner.nextDouble());
    }

    @Override
    public int numDims()
    {
        return 1;
    }

    @Override
    public void createFromDims(double[] dims)
    {
        this.side(dims[0]);
    }

    /**
     * Print the EquilateralTriangle.
     */
    @Override
    public String toString()
    {
        return String.format(FMT_STR, "Name", this.name())
             + String.format(FMT_DBL, "Side", this._side_a)
             + String.format(FMT_DBL, "Height", this.height())
             + String.format(FMT_DBL, "Perimeter", this.perimeter())
             + String.format(FMT_DBL, "Area", this.area());
    }
}





