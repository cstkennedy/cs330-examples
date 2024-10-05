// Thomas Kennedy
// CS 330 Fall 2020

package edu.odu.cs.cs330.examples.shapes;

/**
 * A Rectangle with 4 Equal Sides.
 *
 * @author Thomas J Kennedy
 */
public class Square implements Shape {
    /**
     * Length of One Side.
     */
    private double _side;

    /**
     * Construct a Square with side set to 1.
     */
    public Square()
    {
        this(1);
    }

    /**
     * Construct a Square.
     *
     * @param s the desired side length
     */
    public Square(double s)
    {
        _side = s;
    }

    /**
     * Return the side length.
     *
     * @return length of a single side
     */
    public double side()
    {
        return _side;
    }

    /**
     * Modify the side length.
     *
     * @param s the replacement length
     */
    public void side(double s)
    {
        _side = s;
    }

    @Override
    public String name()
    {
        return "Square";
    }

    @Override
    public double area()
    {
        return Math.pow(_side, 2.0);
    }

    @Override
    public double perimeter()
    {
        return 4 * _side;
    }

    @Override
    public Object clone() throws CloneNotSupportedException
    {
        return new Square(this.side());
    }

    @Override
    public int numDims()
    {
        return 1;
    }

    @Override
    public void createFromDims(double[] dims)
    {
        this._side = dims[0];
    }

    @Override
    public String toString()
    {
        return String.join(
            "",
            String.format(Shape.FMT_STR, "Name", this.name()),
            String.format(Shape.FMT_DBL, "Side", this._side),
            String.format(Shape.FMT_DBL, "Perimeter", this.perimeter()),
            String.format(Shape.FMT_DBL, "Area", this.area())
        );
    }
}





