package house;

import java.util.Collection;
import java.util.Iterator;
import containers.LinkedList;

/**
 * A House is composed of zero or more Room objects.
 * <p>
 * This class serves as our demonstration of the
 * iterator interface.
 */
@SuppressWarnings({
    "PMD.DataflowAnomalyAnalysis",
    "PMD.LawOfDemeter",
    "PMD.MethodArgumentCouldBeFinal",
    "PMD.OnlyOneReturn"
})
public class House implements Iterable<Room>
{
    /**
     * Name of the house--e.g., Minecraft Beach House.
     */
    private String name;

    // private LinkedList<Room> rooms;
    final private Collection<Room> rooms;

    /**
     * Construct a House with a generic name and no rooms.
     */
    public House()
    {
        this.name = "Generic";
        this.rooms = new LinkedList<>();
        // this.rooms = new ArrayList<>();
    }

    /**
     * Construct a House with a specified name.
     */
    public House(String name)
    {
        this.name = name;
        this.rooms = new LinkedList<>();
        // this.rooms = new ArrayList<>();
    }

    /**
     * Add a Room.
     *
     * @param toAdd new Room object
     */
    public void addRoom(Room toAdd)
    {
        this.rooms.add(toAdd);
    }

    /**
     * Update the name.
     */
    public void setName(String newName)
    {
        this.name = newName;
    }

    /**
     * Get the name.
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
    @Override
    public Iterator<Room> iterator()
    {
        return this.rooms.iterator();
    }

    /**
     * Return the size of the house--i.e.,
     * the number of rooms.
     */
    public int size()
    {
        return this.rooms.size();
    }

    @Override
    public boolean equals(Object rhsObj)
    {
        if (!(rhsObj instanceof House)) {
            return false;
        }

        final House rhs = (House) rhsObj;

        if (!this.name.equals(rhs.name)) {
            return false;
        }

        if (this.size() != rhs.size()) {
            return false;
        }

        final Iterator<Room> lhsIt = this.iterator();
        final Iterator<Room> rhsIt = rhs.iterator();

        // We know that both houses have the same number of rooms.
        while (lhsIt.hasNext()) {
            final Room lhsRoom = lhsIt.next();
            final Room rhsRoom = rhsIt.next();

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

        final double avg   = totalCost / this.size();

        toOutput += String.format("------------------------------%n");
        toOutput += String.format("Total Cost   : $ %.2f%n", totalCost);
        toOutput += String.format("Avg Room Cost: $ %.2f", avg);

        return toOutput;
    }
}
