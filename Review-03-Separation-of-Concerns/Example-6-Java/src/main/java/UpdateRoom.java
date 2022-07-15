import java.io.BufferedReader;
import java.io.StringReader;
import java.io.IOException;
import java.util.Scanner;

import house.Room;
import house.House;

public class UpdateRoom
{

    public static final String ROOM_DATA = "Laundry Room; 8 4 1.95 Laminate\n"
            + "Kitchen; 20 12 3.87 Tile\n"
            + "Storage Room; 16 16 4.39 Birch Wood";

    /**
     * Build our example house
     */
    static void buildHouse(BufferedReader reader, House house)
        throws IOException
    {
        String line;

        while ((line = reader.readLine()) != null) {
            int idxSemicolon = line.indexOf(';');

            String name = line.substring(0, idxSemicolon);
            String theRest = line.substring(idxSemicolon + 1, line.length());

            Scanner withinLineScanner = new Scanner(theRest);
            double length = withinLineScanner.nextDouble();
            double width = withinLineScanner.nextDouble();
            double unitCost = withinLineScanner.nextDouble();
            String type = withinLineScanner.nextLine();

            Room aRoom = new Room(name,
                                  new Room.DimensionSet(length, width),
                                  unitCost,
                                  type);

            house.addRoom(aRoom);
        }
    }

    /**
     * Take a room and change the flooring
     *
     * @param original House to change
     *
     * @return House with the updated flooring
     */
    static House upgradeFlooring(House original)
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
    static double discountFlooring(Room r, double percent)
    {
        final double scale = 1 - percent;

        return scale * r.flooringCost();
    }

    /**
     * Compute the area of a room and the cost of
     * flooring for the room
     * <p>
     * Let us Review the use of reference and pointer variables.
     * <p>
     * We will use these when we implement the iterator interface.
     */
    public static void main(String... args)
        throws IOException
    {
        // Construct, build, and print a house
        House house = new House();

        BufferedReader fakeInputFile = new BufferedReader(new StringReader(ROOM_DATA));
        buildHouse(fakeInputFile, house);

        System.out.println(house);
        System.out.println();

        // Upgrade the flooring in a second duplicate house
        House duplicateHouse = upgradeFlooring(house);

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
        /*

        // Get all the flooring costs with a 10% discount
        auto discountFunc = std::bind(discountFlooring, std::placeholders::_1, 0.1);

        vector<double> costs(duplicateHouse.size());
        std::transform(duplicateHouse.begin(), duplicateHouse.end(), costs.begin(),
                       discountFunc);

        std::copy(costs.begin(), costs.end(),
                  std::ostream_iterator<double>(std::cout, "\n"));

        // Print the sum, min, max -> D.R.Y!
        cout << "Total: "
             << std::accumulate(costs.begin(), costs.end(), 0.0, std::plus<double>())
             << "\n";
        cout << "Min: "
             << *std::min_element(costs.begin(), costs.end())
             << "\n"
             << "Max: "
             << *std::max_element(costs.begin(), costs.end())
             << "\n";
        */

        // I would probably use minmax_element and auto
        /*
        std::pair<std::vector<double>::const_iterator,
                  std::vector<double>::const_iterator> extremes = std::minmax_element(costs.begin(), costs.end());
        */

        /*
        auto extremes = std::minmax_element(costs.begin(), costs.end());

        cout << "Min: " << *(extremes.first)  << "\n"
             << "Max: " << *(extremes.second) << "\n";

        return 0;
        */
    }
}
