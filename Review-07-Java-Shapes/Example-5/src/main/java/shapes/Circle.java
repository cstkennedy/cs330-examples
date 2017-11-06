// Thomas Kennedy
// CS 330 Fall 2014

package shapes;

import java.util.Scanner;

/**
 * An Ellipse with equivlant major and minor axes
 */
public class Circle extends Shape { 
    private static final double PI  = Math.PI; ///< \f$ \pi \f$
    private static final double TAU = 2 * PI;  ///< \f$ \tau = 2\pi \f$

    private double _radius; ///< Length of the radius

    /**
     * Construct a Circle with radius set to 1
     */
    public Circle()
    {
        this._name   = "Circle";
        this._radius = 1;
    }

    /**
     * Construct a Circle
     *  
     * @param s the desired radius length
     */
    public Circle( double r )
    {
        this._name   = "Circle";
        this._radius = r;
    }

    /**
     * Construct a Circle
     *
     * @param src the Circle to copy       
     */ 
    public Circle( Circle src )
    {
        this._name   = src._name;
        this._radius = src._radius;
    }

    /**
     * Return the radius length
     */
    public double radius()
    {
        return _radius;        
    }

    /**
     * Modify the radius length
     *
     * @param r the replacement length
     */
    public void radius( double r )
    {
        _radius = r;
    }

    /**
     * Return the diameter
     */
    public double diameter()
    {
        return (
            2 * _radius
        );
    }

    /**
     * Compute the area using \f$ \pi r^2 \f$
     *
     * @return area
     */
    public double area()
    {
        return (
            PI * _radius * _radius
        );
    }

    /**
     * Compute the perimeter using \f$ 2 \pi r \f$
     *
     * @return perimeter
     */
    public double perimeter()
    {
        return (
            TAU * _radius
        );
    }
    
    /**
     * Return a new duplicate Circle
     */
    public Shape clone()
    {
        return new Circle( this );
    }

    /**
     * Read the shape
     *
     * @param scanner the input stream--scanner in this example
     */
    public void read(Scanner scanner)
    {
        this._radius = scanner.nextDouble();
    }

    /**
     * Print the Circle
     *
     * @param outs the output stream--i.e., destination
     */
    public String toString()
    {       
        return (
            String.format( getFormat( "s\n"   ), "Name",      this._name       ) +
            String.format( getFormat( ".4f\n" ), "Radius",    this.radius()    ) +
            String.format( getFormat( ".4f\n" ), "Diameter",  this.diameter()  ) +
            String.format( getFormat( ".4f\n" ), "Perimeter", this.perimeter() ) +
            String.format( getFormat( ".4f\n" ), "Area",      this.area()      )
        );        
    }
}
