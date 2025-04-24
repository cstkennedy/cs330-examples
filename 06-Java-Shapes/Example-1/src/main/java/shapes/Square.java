// Thomas Kennedy
// CS 330 Fall 2014

package shapes;

/**
 * A Rectangle with 4 Equal Sides
 */
public class Square extends Shape
{
    private double _side; ///< Length of One Side

    /**
     * Construct a Square with side set to 1
     */
    public Square()
    {
        super("Square");
        _side = 1;
    }

    /**
     * Construct a Square
     *
     * @param s the desired side length
     */
    public Square(double s)
    {
        super("Square");
        _side = s;
    }

    /**
     * Construct a Square
     *
     * @param src the Square to copy
     */
    public Square(Square src)
    {
        super("Square");
        this._side = src._side;
    }

    /**
     * Return the side length
     */
    public double side()
    {
        return _side;
    }

    /**
     * Modify the side length
     *
     * @param s the replacement length
     */
    public void side(double s)
    {
        _side = s;
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
    public Shape clone()
    {
        return new Square(this);
    }

    @Override
    public String toString()
    {
        return super.toString()
             + String.format(FMT_DBL, "Side", this._side)
             + String.format(FMT_DBL, "Perimeter", this.perimeter())
             + String.format(FMT_DBL, "Area", this.area());
    }
}





