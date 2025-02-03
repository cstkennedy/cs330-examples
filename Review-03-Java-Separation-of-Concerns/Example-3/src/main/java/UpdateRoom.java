import java.io.BufferedReader;
import java.io.StringReader;
import java.io.IOException;
import java.util.Arrays;

import house.Room;
import house.House;
import house.HouseIO;

@SuppressWarnings({
    "PMD.LawOfDemeter",
    "PMD.NoPackage",
    "PMD.SystemPrintln"
})
public class UpdateRoom
{
    public static final String ROOM_DATA = String.join(
        System.lineSeparator(),
        "Laundry Room; 8 4 1.95 Laminate",
        "Kitchen; 20 12 3.87 Tile",
        "Storage Room; 16 16 4.39 Birch Wood"
    );

    /**
     * Take a room and change the flooring.
     *
     * @param original House to change
     *
     * @return House with the updated flooring
     */
    @SuppressWarnings({
        "PMD.LawOfDemeter"
    })
    public static House upgradeFlooring(final House original)
    {
        final House modified = new House();
        modified.setName("After Stone Bricks");

        for (final Room rm : original) {
            final Room copy = rm.clone();
            copy.setFlooring("Stone Bricks", 12.97);

            modified.addRoom(copy);
        }

        return modified;
    }

    /**
     * Compute the discounting flooring price for a single Room.
     *
     * @param room room to examine
     * @param percent discount as a decimal
     *
     * @return discounted flooring price
     *
     * @pre 0 <= percent && percent <= 1
     */
    public static double discountFlooring(final Room room, final double percent)
    {
        final double scale = 1 - percent;

        return scale * room.flooringCost();
    }

    /**
     * Extract and discount renovation cost of each room.
     *
     * @param house House from which to extract room costs
     *
     * @return collection of costs
     */
    @SuppressWarnings({
        "PMD.DataflowAnomalyAnalysis"
    })
    public static double[] extractRoomCosts(final House house)
    {
        final double[] costs = new double[house.size()];

        int idx = 0;
        for (final Room originalRoom : house) {
            costs[idx] = discountFlooring(originalRoom, 0.1);
            ++idx;
        }

        return costs;
    }

    /**
     * Compute the area of a room and the cost of flooring for the room.
     */
    public static void main(final String... args)
        throws IOException
    {
        //----------------------------------------------------------------------
        // Set up input variables
        //----------------------------------------------------------------------
        final StringReader inputStringReader = new StringReader(ROOM_DATA);
        final BufferedReader fakeInputFile = new BufferedReader(inputStringReader);

        //----------------------------------------------------------------------
        // Construct, build, and print a house
        //----------------------------------------------------------------------
        final House house = HouseIO.buildHouse(fakeInputFile);

        System.out.println(house);
        System.out.println();

        //----------------------------------------------------------------------
        // Upgrade the flooring in a second duplicate house
        //----------------------------------------------------------------------
        final House duplicateHouse = upgradeFlooring(house);

        //----------------------------------------------------------------------
        // Demo "equals" method and object variables
        //----------------------------------------------------------------------
        System.out.printf(
            "house == duplicateHouse     -> %b%n",
             house.equals(duplicateHouse)
        );
        System.out.printf(
            "&house == &duplicateHouse   -> %b%n",
             house == duplicateHouse
        );
        System.out.println();

        // Which output is "more readable?"
        System.out.printf("%s%n%n", house);
        System.out.println(duplicateHouse);
        System.out.println();

        //----------------------------------------------------------------------
        // Create, populate, and output an array of room costs
        //----------------------------------------------------------------------
        final double[] costs = extractRoomCosts(duplicateHouse);

        for (final double cost : costs) {
            System.out.printf("%.2f%n", cost);
        }

        //----------------------------------------------------------------------
        // Compute and output cost statistics
        //----------------------------------------------------------------------
        final double total = Arrays.stream(costs).sum();
        final double min = Arrays.stream(costs).min().getAsDouble();
        final double max = Arrays.stream(costs).max().getAsDouble();

        System.out.println();
        System.out.printf("Total: %10.2f%n", total);
        System.out.printf("Min  : %10.2f%n", min);
        System.out.printf("Max  : %10.2f%n", max);
    }
}
