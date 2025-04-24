// Thomas Kennedy
// CS 330 Fall 2020

package edu.odu.cs.cs330.examples.shapes;

import java.util.Scanner;

/**
 * A Polygon with 3 Sides.
 *
 * @author Thomas J Kennedy
 */
public class Triangle extends Shape implements Cloneable {

    /**
     * Length of Side A.
     */
    protected double _side_a;

    /**
     * Length of Side B.
     */
    protected double _side_b;

    /**
     * Length of Side C.
     */
    protected double _side_c;

    /**
     * Create a Triangle with side of length 1.
     */
    public Triangle()
    {
        this(1, 1, 1);
    }

    /**
     * Construct a triangle.
     *
     * @param _side_a the length of side A
     * @param _side_b the length of side B
     * @param _side_c the length of side C
     *
     */
    public Triangle(double _side_a, double _side_b, double _side_c)
    {
        this._side_a = _side_a;
        this._side_b = _side_b;
        this._side_c = _side_c;
    }

    /**
     * Construct a Triangle.
     *
     * @param src the Triangle to copy
     */
    public Triangle(Triangle src)
    {
        this._side_a = src._side_a;
        this._side_b = src._side_b;
        this._side_c = src._side_c;
    }

    /**
     * Length of side A.
     *
     * @return the length of side A
     */
    public double sideA()
    {
        return _side_a;
    }

    /**
     * Modify the length of side A.
     *
     * @param side the replacement length
     */
    public void sideA(double side)
    {
        this._side_a = side;
    }

    /**
     * Length of side B.
     *
     * @return the length of side B
     */
    public double sideB()
    {
        return _side_b;
    }

    /**
     * Modify the length of side B.
     *
     * @param side the replacement length
     */
    public void sideB(double side)
    {
        this._side_b = side;
    }

    /**
     * Length of side C.
     *
     * @return the length of side C
     */
    public double sideC()
    {
        return _side_c;
    }

    /**
     * Modify the length of side C.
     *
     * @param side the replacement length
     */
    public void sideC(double side)
    {
        this._side_c = side;
    }

    @Override
    public String name()
    {
        return "Triangle";
    }

    /**
     * Compute the area using Heron's Formula.
     * Use @f$ s = \frac{1}{2}Perimeter @f$ and
     * @f$ Area = \sqrt{ s(s-a)(s-b)(s-c) } @f$
     *
     * @return the area
     */
    @Override
    public double area()
    {
        double s = perimeter() / 2;

        return (Math.sqrt(
            s * (s - sideA())
              * (s - sideB())
              * (s - sideC())
       ));
    }

    /**
     * Compute the perimeter.
     * @f$ side_a + side_b + side_c @f$
     *
     * @return the perimeter
     */
    @Override
    public double perimeter()
    {
        return (
            _side_a + _side_b + _side_c
       );
    }

    @Override
    public Object clone() throws CloneNotSupportedException
    {
        return new Triangle(this);
    }

    @Override
    public void read(Scanner scanner)
    {
        this._side_a = scanner.nextDouble();
        this._side_b = scanner.nextDouble();
        this._side_c = scanner.nextDouble();
    }

    @Override
    public int numDims()
    {
        return 3;
    }

    @Override
    public void createFromDims(double[] dims)
    {
        this._side_a = dims[0];
        this._side_b = dims[1];
        this._side_c = dims[2];
    }

    @Override
    public String toString()
    {
        return String.format(FMT_STR, "Name", this.name())
             + String.format(FMT_DBL, "Side A", this._side_a)
             + String.format(FMT_DBL, "Side B", this._side_b)
             + String.format(FMT_DBL, "Side C", this._side_c)
             + String.format(FMT_DBL, "Perimeter", this.perimeter())
             + String.format(FMT_DBL, "Area", this.area());
    }
}
