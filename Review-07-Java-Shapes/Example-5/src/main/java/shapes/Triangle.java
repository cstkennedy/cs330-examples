// Thomas Kennedy
// CS 330 Fall 2014

package shapes;

import java.util.Scanner;

/**
 * A Polygon with 3 Sides
 */
public class Triangle extends Shape {

    protected double _side_a; ///< Length of Side A
    protected double _side_b; ///< Length of Side B
    protected double _side_c; ///< Length of Side C

    /**
     * Create a Triangle with side of length 1
     */
    public Triangle()
    {
        super( "Triangle" );

        this._side_a = 1;
        this._side_b = 1;
        this._side_c = 1;
    }

    /**
     * Construct a triangle
     *
     * @param _side_a the length of side A
     * @param _side_b the length of side B
     * @param _side_c the length of side C
     *
     */
    public Triangle( double _side_a, double _side_b, double _side_c )
    {
        super( "Triangle" );

        this._side_a = _side_a;
        this._side_b = _side_b;
        this._side_c = _side_c;
    }

    /**
     * Construct a Triangle
     *
     * @param src the Triangle to copy       
     */
    public Triangle( Triangle src )
    {
        super( "Triangle" );

        this._side_a = src._side_a;
        this._side_b = src._side_b;
        this._side_c = src._side_c;
    }

    /**
     * Length of side A
     *
     * @return the length of side A
     */
    public double sideA()
    {
        return _side_a;
    }

    /**
     * Modify the length of side A
     *
     * @param side the replacement length
     */
    public void sideA( double side )
    {
        this._side_a = side;
    }

    /**
     * Length of side B
     *
     * @return the length of side B
     */
    public double sideB()
    {
        return _side_b;
    }

    /**
     * Modify the length of side B
     *
     * @param side the replacement length
     */
    public void sideB( double side )
    {
        this._side_b = side;
    }

    /**
     * Length of side C
     *
     * @return the length of side C
     */
    public double sideC()
    {
        return _side_c;
    }

    /**
     * Modify the length of side C
     *
     * @param side the replacement length
     */
    public void sideC( double side )
    {
        this._side_c = side;
    }

    /**
     * Compute the area using Heron's Formula.
     * Use @f$ s = \frac{1}{2}Perimeter @f$ and
     * @f$ Area = \sqrt{ s(s-a)(s-b)(s-c) } @f$
     *
     * @return the area
     */
    public double area()
    {
        double s = perimeter() / 2;

        return ( Math.sqrt( 
            s * ( s - sideA() ) 
              * ( s - sideB() ) 
              * ( s - sideC() )
        ) );
    }

    /**
     * Compute the perimeter
     * @f$ side_a + side_b + side_c @f$
     *
     * @return the perimeter
     */
    public double perimeter()
    {
        return (
            _side_a + _side_b + _side_c
        );
    }

    /**
     * Return a new duplicate Triangle
     */
    public Shape clone()
    {
        return new Triangle( this );
    }

    /**
     * Read the shape
     *
     * @param scanner the input stream--scanner in this example
     */
    public void read(Scanner scanner)
    {
        this._side_a = scanner.nextDouble();
        this._side_b = scanner.nextDouble();
        this._side_c = scanner.nextDouble();
    }

    /**
     * Print the Triangle
     */
    public String toString()
    {
        return (
            String.format( getFormat( "s\n"   ), "Name",      this._name       ) +
            String.format( getFormat( ".4f\n" ), "Side A",    this._side_a     ) +
            String.format( getFormat( ".4f\n" ), "Side B",    this._side_b     ) +
            String.format( getFormat( ".4f\n" ), "Side C",    this._side_c     ) +
            String.format( getFormat( ".4f\n" ), "Perimeter", this.perimeter() ) +
            String.format( getFormat( ".4f\n" ), "Area",      this.area()      )
        );
    }
};

