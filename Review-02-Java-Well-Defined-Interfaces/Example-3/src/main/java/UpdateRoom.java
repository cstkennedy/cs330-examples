import java.io.BufferedReader;
import java.io.StringReader;
import java.io.IOException;
import java.util.Iterator;
import java.util.Arrays;

import house.Room;
import house.House;

public class UpdateRoom
{
    /**
     * Generate a House with three hardcoded rooms.
     *
     * @return a House containing a laundry room, kitchen and storage room
     */
    public static House buildHouse()
    {
        House house = new House();

        // Add the Laundry Room
        house.addRoom(
            new Room("Laundry Room",
                new Room.DimensionSet(8, 4),
                1.95,
                "Laminate"
            )
        );

        // Add the Kitchen
        house.addRoom(
            new Room(
                "Kitchen",
                new Room.DimensionSet(20, 12),
                3.87,
                "Tile"
            )
        );

        // Add the Storage Room
        house.addRoom(
            new Room(
                "Storage Room",
                new Room.DimensionSet(16, 16),
                4.39,
                "Birch Wood"
            )
        );

        return house;
    }

    /**
     * Compute the area of a room and the cost of flooring for the room.
     */
    public static void main(String... args)
    {
        House house = buildHouse();

        System.out.println(house);

        // What if we decide to use the same type of
        // Flooring in every room?
        for (Room room : house) {
            room.setFlooring("Stone Bricks", 12.97);
        }

        System.out.println();
        house.setName("After Stone Bricks");
        System.out.println(house);
    }
}
