package house;

import java.io.BufferedReader;
import java.io.IOException;
import java.util.Scanner;


/**
 * This is a wrapper for all input and output functions.
 * <p>
 * In a future discussion... we will cover how to write this in a more
 * idiomatic and reusable form.
 */
@SuppressWarnings({
    "PMD.DataflowAnomalyAnalysis",
    "PMD.LawOfDemeter"
})
public class HouseIO
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
        final int idxSemicolon = theString.indexOf(';');

        final String name = theString.substring(0, idxSemicolon);
        final String theRest = theString.substring(idxSemicolon + 1, theString.length());

        Scanner scnr = new Scanner(theRest);

        final double length = scnr.nextDouble();
        final double width = scnr.nextDouble();
        final double unitCost = scnr.nextDouble();
        final String type = scnr.nextLine();

        return new Room(
            name,
            new Room.DimensionSet(length, width),
            unitCost,
            type
        );
    }

    /**
     * Build our example house.
     *
     * @param reader source of House information
     *
     * @return fully constructed House
     */
    public static House buildHouse(final BufferedReader reader)
        throws IOException
    {
        final House house = new House();

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
