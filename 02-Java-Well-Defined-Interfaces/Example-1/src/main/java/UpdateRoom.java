import java.io.BufferedReader;
import java.io.StringReader;
import java.io.IOException;
import java.util.Iterator;
import java.util.Arrays;

import house.Room;

public class UpdateRoom
{
    /**
     * Compute the area of a room and the cost of flooring for the room.
     */
    public static void main(String... args)
    {
        Room room = new Room("Laundry Room", 8, 4, 1.95);
        Room kitchen = new Room("Kitchen", 20, 12, 3.87);

        Room roomPointer = room;

        room.setFlooring("Tile", 2.50);

        System.out.println("----------------------------------------------------------------");
        System.out.println(room);
        System.out.println("----------------------------------------------------------------");

        System.out.println(roomPointer);

        System.out.println("----------------------------------------------------------------");
        System.out.printf(
            "&room (%s)  == &roomPointer (%s)   -> %b%n",
            Integer.toHexString(System.identityHashCode(room)),
            Integer.toHexString(System.identityHashCode(roomPointer)),
            (room == roomPointer)
        );
        System.out.println("----------------------------------------------------------------");

        if (room.equals(roomPointer)) {
            System.out.println("room == roomPointer");
        }
        else {
            System.out.println("room != roomPointer");
        }

        System.out.println("----------------------------------------------------------------");

        if (room.equals(kitchen)) {
            System.out.println("room == kitchen");
        }
        else {
            System.out.println("room != kitchen");
        }

        System.out.println();
    }
}
