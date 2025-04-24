// Thomas Kennedy
// CS 330 Fall 2020

package edu.odu.cs.cs330.examples.shapes;

/**
 * A Triangle with all sides set to the same length.
 *
 * @author Thomas J Kennedy
 */
public class RightTriangle extends Triangle implements Shape {
    /**
     * A constant for one half. @f$ \frac{1}{2} @f$
     */
    private static final double ONE_HALF = 1.0 / 2.0;

    /**
     * Compute the hyptoenuse using:
     * @f$ hypotenuse = \sqrt{base^2 + height^2} @f$.
     *
     * @param base the base of a Right Triangle
     * @param height the height of a Right Triangle
     *
     * @return hypotenuse of a right triangle
     */
    private static double computeHypotenuse(double base, double height)
    {
        return Math.sqrt((base * base) + (height * height));
    }

    /**
     * Construct a RightTriangle
     * with base and height set to 1.
     */
    public RightTriangle()
    {
        this(1, 1);
    }

    /**
     * Construct a RightTriangle.
     *
     * @param base the desired base
     * @param height the desired height
     */
    public RightTriangle(double base, double height)
    {
        this._side_a = base;
        this._side_b = height;
        this._side_c = RightTriangle.computeHypotenuse(_side_a, _side_b);
    }

    /**
     * Construct a RightTriangle.
     *
     * @param src the RightTriangle to copy
     */
    public RightTriangle(RightTriangle src)
    {
        this._side_a = src._side_a;
        this._side_b = src._side_b;
        this._side_c = src._side_c;
    }

    /**
     * Return the base.
     *
     * @return the base
     */
    public double base()
    {
        return _side_a;
    }

    /**
     * Modify the base.
     *
     * @param side the replacement base
     */
    public void base(double side)
    {
        _side_a = side;

        _side_c = RightTriangle.computeHypotenuse(_side_a, _side_b);
    }

    /**
     * Return the height.
     *
     * @return the height
     */
    public double height()
    {
        return _side_b;
    }

    /**
     * Modify the height.
     *
     * @param side the replacement height
     */
    public void height(double side)
    {
        _side_b = side;

        _side_c = RightTriangle.computeHypotenuse(_side_a, _side_b);
    }

    /**
     * Return the hypotenuse.
     *
     * @return the hypotenuse
     */
    public double hypotenuse()
    {
        return _side_c;
    }

    @Override
    public String name()
    {
        return "Right Triangle";
    }

    /**
     * Compute the area using
     * @f$ Area = \frac{1}{2}*base*height @f$.
     *
     * @return the area
     */
    @Override
    public double area()
    {
        return ONE_HALF * _side_a * _side_b;
    }

    /**
     * Return a new duplicate RightTriangle.
     */
    @Override
    public Object clone() throws CloneNotSupportedException
    {
        RightTriangle cpy = new RightTriangle();
        cpy.height(this.height());
        cpy.base(this.base());

        return cpy;
    }

    @Override
    public int numDims()
    {
        return 2;
    }

    @Override
    public void createFromDims(double[] dims)
    {
        this.base(dims[0]);
        this.height(dims[1]);

        this._side_c = computeHypotenuse(this.base(), this.height());
    }

    /**
     * Print the RightTriangle.
     */
    @Override
    public String toString()
    {
        return String.join(
            "",
            String.format(Shape.FMT_STR, "Name", this.name()),
            String.format(Shape.FMT_DBL, "Base", this.base()),
            String.format(Shape.FMT_DBL, "Height", this.height()),
            String.format(Shape.FMT_DBL, "Hypotenuse", this.hypotenuse()),
            String.format(Shape.FMT_DBL, "Perimeter", this.perimeter()),
            String.format(Shape.FMT_DBL, "Area", this.area())
        );
    }
}





