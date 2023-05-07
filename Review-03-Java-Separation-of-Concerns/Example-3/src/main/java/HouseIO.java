import java.io.BufferedReader;
import java.io.IOException;
import java.util.Scanner;
import java.util.Iterator;

import house.Room;
import house.House;

/**
 * This is a wrapper for all input and output functions.
 * <p>
 * In a future discussion... we will cover how to write this in a more
 * idiomatic and reusable form.
 */
class HouseIO
{
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

        reader
            .lines()
            .map((String line) -> {
                return extractRoomFrom(line);
            })
            .forEach((Room aRoom) -> {
                house.addRoom(aRoom);
            });

        return house;
    }
}
