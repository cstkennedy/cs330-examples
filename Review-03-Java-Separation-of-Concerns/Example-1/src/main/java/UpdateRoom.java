import java.io.BufferedReader;
import java.io.StringReader;
import java.io.IOException;
import java.util.Scanner;
import java.util.Iterator;
import java.util.Arrays;

import house.Room;
import house.House;

public class UpdateRoom
{
    public static final String ROOM_DATA = String.join(
        System.lineSeparator(),
        "Laundry Room; 8 4 1.95 Laminate",
        "Kitchen; 20 12 3.87 Tile",
        "Storage Room; 16 16 4.39 Birch Wood"
    );

    /**
     * Extract Room information from a string.
     *
     * @param theString input string containing the data for exactly one Room
     *
     * @return initialized Room object
     */
    public static Room extractRoomFrom(String theString)
    {
        int idxSemicolon = theString.indexOf(';');

        String name = theString.substring(0, idxSemicolon);
        String theRest = theString.substring(idxSemicolon + 1, theString.length());

        Scanner scnr = new Scanner(theRest);

        double length = scnr.nextDouble();
        double width = scnr.nextDouble();
        double unitCost = scnr.nextDouble();
        String type = scnr.nextLine();

        Room aRoom = new Room(
            name,
            new Room.DimensionSet(length, width),
            unitCost,
            type
        );

        return aRoom;
    }

    /**
     * Build our example house.
     *
     * @param reader source of House information
     *
     * @return fully constructed House
     */
    public static House buildHouse(BufferedReader reader)
        throws IOException
    {
        House house = new House();

        // BufferedReader input loop
        /*
        String line;

        while ((line = reader.readLine()) != null) {
            Room aRoom = extractRoomFrom(line);

            house.addRoom(aRoom);
        }
        */

        // BufferedReader using a "stream"
        reader.lines()
            .map((String line) -> {
                return extractRoomFrom(line);
            })
            .forEach((Room aRoom) -> {
                house.addRoom(aRoom);
            });

        return house;
    }

    /**
     * Take a room and change the flooring
     *
     * @param original House to change
     *
     * @return House with the updated flooring
     */
    public static House upgradeFlooring(House original)
    {
        House modified = new House();
        modified.setName("After Stone Bricks");

        for (Room rm : original) {
            Room copy = rm.clone();
            copy.setFlooring("Stone Bricks", 12.97);

            modified.addRoom(copy);
        }

        return modified;
    }

    /**
     * Compute the discounting flooring price for a single Room.
     *
     * @param r room to examine
     * @param percent discount as a decimal
     *
     * @pre 0 <= percent && percent <= 1
     */
    public static double discountFlooring(Room r, double percent)
    {
        final double scale = 1 - percent;

        return scale * r.flooringCost();
    }

    /**
     * Extract and discount renovation cost of each room.
     *
     * @param house House from which to extract room costs
     *
     * @return collection of costs
     */
    public static double[] extractRoomCosts(House house)
    {
        double[] costs = new double[house.size()];

        Iterator<Room> it = house.iterator();
        for (int i = 0; i < house.size(); ++i) {
            costs[i] = discountFlooring(it.next(), 0.1);
        }

        return costs;
    }

    /**
     * Compute the area of a room and the cost of flooring for the room.
     */
    public static void main(String... args)
        throws IOException
    {
        //----------------------------------------------------------------------
        // Set up input variables
        //----------------------------------------------------------------------
        StringReader inputStringReader = new StringReader(ROOM_DATA);
        BufferedReader fakeInputFile = new BufferedReader(inputStringReader);

        //----------------------------------------------------------------------
        // Construct, build, and print a house
        //----------------------------------------------------------------------
        House house = buildHouse(fakeInputFile);

        System.out.println(house);
        System.out.println();

        //----------------------------------------------------------------------
        // Upgrade the flooring in a second duplicate house
        //----------------------------------------------------------------------
        House duplicateHouse = upgradeFlooring(house);

        //----------------------------------------------------------------------
        // Demo "equals" method and object variables
        //----------------------------------------------------------------------
        System.out.printf(
            "house == duplicateHouse     -> %b%n",
             (house.equals(duplicateHouse))
        );
        System.out.printf(
            "&house == &duplicateHouse   -> %b%n",
             (house == duplicateHouse)
        );
        System.out.println();

        System.out.println(house);
        System.out.println();
        System.out.println(duplicateHouse);
        System.out.println();

        //----------------------------------------------------------------------
        // Create, populate, and output an array of room costs
        //----------------------------------------------------------------------
        double[] costs = extractRoomCosts(duplicateHouse);

        for (double cost : costs) {
            System.out.printf("%.2f%n", cost);
        }

        //----------------------------------------------------------------------
        // Compute and output cost statistics
        //----------------------------------------------------------------------
        double total = Arrays.stream(costs).sum();
        double min = Arrays.stream(costs).min().getAsDouble();
        double max = Arrays.stream(costs).max().getAsDouble();

        System.out.println();
        System.out.printf("Total: %10.2f%n", total);
        System.out.printf("Min  : %10.2f%n", min);
        System.out.printf("Max  : %10.2f%n", max);
    }
}
