package house;

import java.util.Collection;
import java.util.Iterator;
import java.util.ArrayList;

/**
 * A House is composed of zero or more Room objects.
 * <p>
 * This class serves as our demonstration of the
 * iterator interface.
 */
public class House implements Iterable<Room>
{
    /**
     * Name of the house--e.g.,
     * Minecraft Beach House
     */
    private String name;

    // private LinkedList<Room> rooms;
    private Collection<Room> rooms;

    /**
     * Construct a House with a
     * generic name and no rooms.
     */
    public House()
    {
        this.name = "Generic";
        this.rooms = new ArrayList<>();
    }

    /**
     * Construct a House with a specified name
     */
    public House(String name)
    {
        this.name = name;
        this.rooms = new ArrayList<>();
    }

    /**
     * Add a Room
     *
     * @param toAdd new Room object
     */
    public void addRoom(Room toAdd)
    {
        this.rooms.add(toAdd);
    }

    /**
     * Update the name
     */
    public void setName(String newName)
    {
        this.name = newName;
    }

    /**
     * Update the name
     */
    public String getName()
    {
        return this.name;
    }

    /**
     * Allow access to the _beginning_ of the
     * house--i.e., Room container--via an
     * iterator.
     */
    public Iterator<Room> iterator()
    {
        return this.rooms.iterator();
    }

    /**
     * Return the size of the house--i.e.,
     * the number of rooms
     */
    public int size()
    {
        return this.rooms.size();
    }

    /**
     * Logical Equivalence Operator
     */
    @Override
    public boolean equals(Object rhsObj)
    {
        if (!(rhsObj instanceof House)) {
            return false;
        }

        House rhs = (House) rhsObj;

        if (!this.name.equals(rhs.name)) {
            return false;
        }

        if (this.size() != rhs.size()) {
           return false;
        }

        Iterator<Room> lhsIt = this.iterator();
        Iterator<Room> rhsIt = rhs.iterator();

        // We know that both houses have the same number of rooms.
        while (lhsIt.hasNext()) {
            Room lhsRoom = lhsIt.next();
            Room rhsRoom = rhsIt.next();

            if (!lhsRoom.equals(rhsRoom)) {
                return false;
            }
        }

        return true;
    }

    @Override
    public String toString()
    {
        // I will use String concatenation in this example. We will discuss the
        // **proper** and efficient** way to combine strings in Java in a future
        // example.

        String toOutput = String.format("--------%s--------%n", this.name);

        double totalCost = 0;
        for (Room rm : this) {
            toOutput += rm + String.format("%n");

            totalCost += rm.flooringCost();
        }

        double avg   = totalCost / this.size();

        toOutput += String.format("------------------------------%n");
        toOutput += String.format("Total Cost   : $ %.2f%n", totalCost);
        toOutput += String.format("Avg Room Cost: $ %.2f", avg);

        return toOutput;
    }
}
