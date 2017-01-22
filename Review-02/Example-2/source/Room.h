#ifndef ROOM_H_DEFINED
#define ROOM_H_DEFINED

#include <iostream>
#include <string>
#include <utility>

using namespace std::rel_ops;

/**
 * Monetary cost. Note that in a non-academic setting,
 * this would likely be represented by a more robust
 * Money ADT--or API.
 */
typedef double Cost;

/**
 * A Room Blueprint. This struct, defines
 * a room. For the moment this is simply
 * a grouping of attributes (variables)
 * that describe a Room
 */
class Room {
    public:
        /**
         * Units of length--e.g., meters
         */
        static const std::string UNITS;

        /**
         * Flooring Record for a Room. Note
         * that this data-type is meaningless
         * outside the context of of Room ADT
         * for this scenario.
         */
        struct Flooring {
            std::string type;
            Cost        unitCost;

            /**
             * Default Constructor
             */
            Flooring();

            /**
             * Non-Default Constructor
             */
            Flooring(std::string n, Cost c);
        };

        /**
         * One linear dimension. This can be one of
         * length, width, or height
         */
        typedef double Dimension;

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
        class DimensionSet {
            private:
                Dimension  length;
                Dimension   width;

            public:
                /**
                 * Default to dimensions of 1
                 */
                DimensionSet();

                /**
                 * Set the length and width to user 
                 * specified values
                 */
                DimensionSet(Dimension l, Dimension w);

                /**
                 * Set the length
                 *
                 * @param v replacement value
                 */
                void setLength(Dimension v);

                /**
                 * Retrieve the length
                 */
                Dimension getLength() const;

                /**
                 * Set the width
                 *
                 * @param v replacement value
                 */
                void setWidth(Dimension v);

                /**
                 * Retrieve the width
                 */
                Dimension getWidth() const;
        };

    private:
        /**
         * This is the DimensionSet object--i.e, instance.
         */
        DimensionSet dimensions;

        /**
         * This is the Flooring object--i.e., instance
         */
        Flooring     flooring; 

        /**
         * This is the name of the room--i.e., a std::string object
         */
        std::string  name;

    public:
        /**
         * Default Constructor
         */
        Room();

        /**
         * Second, Non-Default Constructor
         *
         * @param l length
         * @param w width
         * @param c cost for 1 sq unit of flooring
         *
         */
        Room(Dimension l, Dimension w, Cost c);

        /**
         * Third, Non-Default constructor
         *
         * @param n name
         * @param l length
         * @param w width
         * @param c cost for 1 sq unit of flooring
         *
         */
        Room(std::string n, Dimension l, Dimension w, Cost c);

        /**
         * Fourth, Non-Default constructor
         *
         * @param n name
         * @param d dimensions
         * @param c cost for 1 sq unit of flooring
         * @param fn flooring type
         *
         */
        Room(std::string n, DimensionSet d, Cost c, std::string fn);

        /**
         * Permit access to the DimensionSet object
         * <p>
         * We will explore this more in a later example.
         * Our emphsis will be on the return type
         */
        const DimensionSet& getDimensions() const;

        /**
         * Allow the dimensions to be changed
         *
         * @param l new length
         * @param w new width
         */
        void setDimensions(Dimension l, Dimension w);

        /**
         * Permit access to the Flooring object
         * <p>
         * We will explore this more in a later example.
         * Our emphsis will be on the return type
         */
        const Flooring& getFlooring() const;

        /**
         * Allow the flooring to be changed
         *
         * @param t flooring type
         * @param c cost per unit
         */
        void setFlooring(std::string t, Cost c);

        /**
         * Set the name
         *
         * @param newName
         */
        void setName(std::string newName);

        /**
         * Retrieve the name
         */
        std::string getName() const;

        /**
         * Compute the area of this room
         */
        double area() const;

        /**
         * Retrive cost of flooring for the entire room
         */
        Cost flooringCost() const;

        /**
         * Generate and display a summary for a single (one) room
         *
         * @param prt Room for which to print the summary
         */
        void display(std::ostream &outs) const;

        /**
         * Logical Equivalence Operator
         * <p>
         * This is the member function implementation.
         * This operator can be implemented as a non-member function.
         */
        bool operator==(const Room &rhs) const;

        /**
         * Less-Than (Comes-Before) Operator.
         * <p>
         * This is used to assign a lexicographical ordering.
         * <p>
         * This is the member function implementation.
         * This operator can be implemented as a non-member function.
         */
        bool operator<(const Room &rhs) const;
};

/**
 *
 */
inline
void Room::DimensionSet::setLength(Dimension v)
{
    this->length = v;
}

/**
 *
 */
inline
Room::Dimension Room::DimensionSet::getLength() const
{
    return this->length;
}

/**
 *
 */
inline
void Room::DimensionSet::setWidth(Dimension v)
{
    this->width = v;
}

/**
 *
 */
inline
Room::Dimension Room::DimensionSet::getWidth() const
{
    return this->width;
}

/**
 *
 */
inline
void Room::setName(std::string newName)
{
    this->name = newName;
}

/**
 * 
 */
inline 
std::string Room::getName() const
{
    return this->name;
}

/**
 *
 */
inline
const Room::Flooring& Room::getFlooring() const
{
    return this->flooring;
}

/**
 *
 */
inline
void Room::setFlooring(std::string t, Cost c)
{
    flooring.type     = t;
    flooring.unitCost = c;
}

/**
 *
 */
inline
const Room::DimensionSet& Room::getDimensions() const
{
    return this->dimensions;
}

/**
 * 
 */
inline
double Room::area() const
{
    return (dimensions.getWidth() * dimensions.getLength());
}

/**
 *
 */
inline
Cost Room::flooringCost() const
{
    return (area() * flooring.unitCost);
}

/**
 * Room Stream Insertion (Output) Operator
 * 
 * This is often written as a wrapper for a 
 * display or print function.
 * <p>
 * This operator can *NOT* be implemented as a member function.
 */
inline
std::ostream& operator<<(std::ostream &outs, const Room &prt)
{
    prt.display(outs);

    return outs;
}
#endif