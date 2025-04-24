// Thomas Kennedy
// CS 330 Fall 2020

package edu.odu.cs.cs330.examples.shapes;

import java.util.Scanner;

/**
 * An Ellipse with equivalent major and minor axes.
 *
 * @author Thomas J Kennedy
 */
public class Circle extends Shape implements Cloneable {
    /**
     * The mathematical constant PI.
     */
    private static final double PI  = Math.PI;

    /**
     * Tau is defined as 2 * PI.
     */
    private static final double TAU = 2 * PI;

    /**
     * Length of the radius.
     */
    private double _radius;

    /**
     * Construct a Circle with radius set to 1.
     */
    public Circle()
    {
        this(1);
    }

    /**
     * Construct a Circle.
     *
     * @param r the desired radius length
     */
    public Circle(double r)
    {
        this._radius = r;
    }

    /**
     * Construct a Circle.
     *
     * @param src the Circle to copy
     */
    public Circle(Circle src)
    {
        this._radius = src._radius;
    }

    /**
     * Return the radius length.
     */
    public double radius()
    {
        return _radius;
    }

    /**
     * Modify the radius length.
     *
     * @param r the replacement length
     */
    public void radius(double r)
    {
        _radius = r;
    }

    /**
     * Return the diameter.
     */
    public double diameter()
    {
        return 2 * _radius;
    }

    @Override
    public String name()
    {
        return "Circle";
    }

    /**
     * Compute the area using \f$ \pi r^2 \f$.
     *
     * @return area
     */
    @Override
    public double area()
    {
        return PI * _radius * _radius;
    }

    /**
     * Compute the perimeter using \f$ 2 \pi r \f$.
     *
     * @return perimeter
     */
    @Override
    public double perimeter()
    {
        return TAU * _radius;
    }

    /**
     * Return a new duplicate Circle.
     */
    @Override
    public Object clone() throws CloneNotSupportedException
    {
        return new Circle(this);
    }

    /**
     * Read the shape.
     *
     * @param scanner the input stream--scanner in this example
     */
    @Override
    public void read(Scanner scanner)
    {
        this._radius = scanner.nextDouble();
    }

    @Override
    public int numDims()
    {
        return 1;
    }

    @Override
    public void createFromDims(double[] dims)
    {
        this._radius = dims[0];
    }

    /**
     * Print the Circle.
     */
    @Override
    public String toString()
    {
        return String.format(FMT_STR, "Name", this.name())
             + String.format(FMT_DBL, "Radius", this.radius())
             + String.format(FMT_DBL, "Diameter", this.diameter())
             + String.format(FMT_DBL, "Perimeter", this.perimeter())
             + String.format(FMT_DBL, "Area", this.area());
    }
}
