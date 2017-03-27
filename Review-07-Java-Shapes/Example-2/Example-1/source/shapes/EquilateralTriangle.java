// Thomas Kennedy
// CS 330 Fall 2014

package shapes;

/**
 * A Triangle with all sides set to the same length
 */
public class EquilateralTriangle extends Triangle {   
    private static final double ROOT_3_DIV_4 = Math.sqrt( 3 ) / 4; ///< @f$ \frac{\sqrt{3}}{4} @f$
    
    /**
     * Construct an EquilateralTriangle
     * with all sides set to 1.
     */
    public EquilateralTriangle()
    {
        this._name = "Equilateral Triangle";
        
        this.side( 1 );
    }

    /**
     * Construct an EquilateralTriangle
     *
     * @param side the desired side length
     */
    public EquilateralTriangle( double side )
    {
        this._name = "Equilateral Triangle";
        
        this.side( side );
    }

    /**
     * Construct an EquilateralTriangle
     *
     * @param src the EquilateralTriangle to copy       
     */ 
    public EquilateralTriangle( EquilateralTriangle src )
    {
        this._name = src._name;

        this.side( src.side() );
    }

    /**
     * Compute the height using
     * @f$ height = \frac{5}{4}side @f$
     *
     * @return height
     */
    public double height()
    {
        return Math.sqrt( 
              1.25 * ( side() * side() )
        );
    }

    /**
     * Return the length of one side
     *
     * @return the length of one side
     */
    public double side()
    {
        return _side_a;
    }

    /**
     * Modify the side length
     *
     * @param s the desired side length
     */
    public void side( double s )
    {
        _side_a = s;
        _side_b = s;
        _side_c = s;
    }

    /**
     * Compute the area using 
     * @f$ Area=\frac{\sqrt{3}}{4}side^2 @f$
     *
     * @return the area
     */
    public double area(){
        return ROOT_3_DIV_4 * side() * side();
    }

    /**
     * Return a new duplicate EquilateralTriangle
     */
    public Shape clone()
    {
        return new EquilateralTriangle( this );
    }

    /**
     * Print the EquilateralTriangle
     */
    public String toString()
    {
        return (
            String.format( getFormat( "s\n"   ), "Name",      this._name       ) +
            String.format( getFormat( ".4f\n" ), "Side",      this._side_a     ) +
            String.format( getFormat( ".4f\n" ), "Height",    this.height()    ) +
            String.format( getFormat( ".4f\n" ), "Perimeter", this.perimeter() ) +
            String.format( getFormat( ".4f\n" ), "Area",      this.area()      )
        );
    }
}





