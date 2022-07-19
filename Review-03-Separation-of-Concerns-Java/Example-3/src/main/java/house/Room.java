package house;

/**
 * A Room Blueprint. This struct, defines
 * a room. For the moment this is simply
 * a grouping of attributes (variables)
 * that describe a Room
 */
public class Room implements Cloneable {
    /**
     * Units of length--e.g., meters
     */
    public static final String UNITS = "ft";

    /**
     * Flooring Record for a Room. Note
     * that this data-type is meaningless
     * outside the context of of Room ADT
     * for this scenario.
     */
    public class Flooring {
        public String type;
        public double unitCost;

        /**
         * Default Constructor
         */
        public Flooring()
        {
            this("Generic", 1.0);
        }

        /**
         * Non-Default Constructor
         */
        public Flooring(String n, double c)
        {
            this.type = n;
            this.unitCost = c;
        }
    }

    /**
     * Container for length and width.
     * <p>
     * This will allow us to reduce the impact
     * of the addition of the height dimension in
     * a later example.
     * <p>
     * For the sake of clarity, I titled this data-type
     * DimensionSet, in practice, I would have more likely
     * named it Dimensions.
     * <p>
     * Note that this is now a proper class.
     */
    public static class DimensionSet {
        private double  length;
        private double   width;

        /**
         * Default to dimensions of 1
         */
        public DimensionSet()
        {
            this(1, 1);
        }

        /**
         * Set the length and width to user
         * specified values
         */
        public DimensionSet(double l, double w)
        {
            this.length = l;
            this.width = w;
        }

        /**
         * Set the length
         *
         * @param v replacement value
         */
        public void setLength(double v)
        {
            this.length = v;
        }

        /**
         * Retrieve the length
         */
        public double getLength()
        {
            return this.length;
        }

        /**
         * Set the width
         *
         * @param v replacement value
         */
        public void setWidth(double v)
        {
            this.width = v;
        }

        /**
         * Retrieve the width
         */
        public double getWidth()
        {
            return this.width;
        }
    }

    /**
     * This is the DimensionSet object--i.e, instance.
     */
    private DimensionSet dimensions;

    /**
     * This is the Flooring object--i.e., instance
     */
    private Flooring flooring;

    /**
     * This is the name of the room--i.e., a String object
     */
    private String  name;

    /**
     * Default Constructor
     */
    public Room()
    {
        this(1, 1, 1);
    }

    /**
     * Second, Non-Default Constructor
     *
     * @param l length
     * @param w width
     * @param c cost for 1 sq unit of flooring
     *
     */
    public Room(double l, double w, double c)
    {
        this("Generic", l, w, c);
    }

    /**
     * Third, Non-Default constructor
     *
     * @param n name
     * @param l length
     * @param w width
     * @param c cost for 1 sq unit of flooring
     *
     */
    public Room(String n, double l, double w, double c)
    {
        this.name = n;
        this.dimensions = new DimensionSet(l, w);
        this.flooring = new Flooring();
        this.flooring.unitCost = c;
    }

    /**
     * Fourth, Non-Default constructor
     *
     * @param n name
     * @param d dimensions
     * @param c cost for 1 sq unit of flooring
     * @param fn flooring type
     *
     */
    public Room(String n, DimensionSet d, double c, String fn)
    {
        this.name = n;
        this.dimensions = d;
        this.flooring = new Flooring(fn, c);
    }

    /**
     * Permit access to the DimensionSet object
     * <p>
     * We will explore this more in a later example.
     * Our emphsis will be on the return type
     */
    public DimensionSet getDimensions()
    {
        return this.dimensions;
    }

    /**
     * Allow the dimensions to be changed
     *
     * @param l new length
     * @param w new width
     */
    public void setDimensions(double l, double w)
    {
        this.dimensions.setLength(l);
        this.dimensions.setWidth(w);
    }

    /**
     * Permit access to the Flooring object
     * <p>
     * We will explore this more in a later example.
     * Our emphsis will be on the return type
     */
    public Flooring getFlooring()
    {
        return this.flooring;
    }

    /**
     * Allow the flooring to be changed
     *
     * @param t flooring type
     * @param c cost per unit
     */
    public void setFlooring(String t, double c)
    {
        this.flooring.type = t;
        this.flooring.unitCost = c;
    }

    /**
     * Set the name
     *
     * @param newName
     */
    public void setName(String newName)
    {
        this.name = newName;
    }

    /**
     * Retrieve the name
     */
    public String getName()
    {
        return this.name;
    }

    /**
     * Compute the area of this room
     */
    public double area()
    {
        return this.dimensions.getLength() * this.dimensions.getWidth();
    }

    /**
     * Retrive cost of flooring for the entire room
     */
    public double flooringCost()
    {
        return this.area() * this.flooring.unitCost;
    }

    /**
     * Generate and display a summary for a single (one) room
     */
    @Override
    public String toString()
    {
        return String.format("Room (%s)%n", this.name)
             + String.format("  Length: %.1f %s%n", this.dimensions.getLength(), Room.UNITS)
             + String.format("  Width : %.1f %s%n", this.dimensions.getWidth(), Room.UNITS)
             + String.format("  Area  : %.1f sq %s%n", this.area(), Room.UNITS)
             + String.format("%n")
             + String.format("  Flooring   : %s%n", this.flooring.type)
             + String.format("  Unit Cost  : $ %8.2f%n", this.flooring.unitCost)
             + String.format("  Total Cost : $ %8.2f%n", this.flooringCost());
    }

    /**
     * Logical Equivalence Operator
     * <p>
     * This is the member function implementation.
     * This operator can be implemented as a non-member function.
     */
    @Override
    public boolean equals(Object rhs)
    {
        if (!(rhs instanceof Room)) {
            return false;
        }

        Room rhsRoom = (Room) rhs;

        return this.name.equals(rhsRoom.name)
            && this.area() == rhsRoom.area();
    }

    public Room clone()
    {
        Room cpy = new Room();
        cpy.setName(this.name);
        cpy.setDimensions(this.dimensions.getLength(), this.dimensions.getWidth());
        cpy.setFlooring(this.flooring.type, this.flooring.unitCost);

        return cpy;
    }
}

/*
//------------------------------------------------------------------------------
inline
std::istream& operator>>(std::istream &ins, Room& rd)
{
    String name;
    double      l, h;
    double      cost;
    String flooring;

    ins >> std::ws;
    getline(ins, name, ';');
    ins >> l >> h >> cost;

    ins >> std::ws;
    getline(ins, flooring);
    ins >> std::ws;

    rd = Room(name, Room::DimensionSet(l, h), cost, flooring);

    return ins;
}
*/
