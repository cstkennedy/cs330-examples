import java.io.BufferedReader;
import java.io.StringReader;
import java.io.IOException;
import java.util.Iterator;
import java.util.Arrays;
import java.util.Random;

import static utilities.Utilities.horizontalLine;
import static utilities.Utilities.projectHeading;

import containers.LinkedList;


public class ListDriver
{
    private static final String[] PROGRAM_HEADING = {
        "Linked List Review",
        "Thomas J. Kennedy"
    };

    /**
     * Width of the "Window"
     *
     * This represents the intended max width of the output.
     */
    private static final int W_WIDTH = 80;

    /**
     * Lower bound for number generation
     */
    public static final int MIN = -10;

    /**
     * Upper bound for number generation
     */
    public static final int MAX = 10;

    /**
     * Default list size
     */
    private static int DEFAULT_NUM_NODES = 10;

    /**
     * Generate a random integer in the range min, max.
     */
    public static int randomInt(Random random, int min, int max)
    {
        int distance = max - min + 1;

        int rawInt = random.nextInt(distance);

        return rawInt + min;
    }

    /**
     * Generate a random integer in the range min, max.
     * Default to MIN and MAX
     */
    public static int randomInt(Random random)
    {
        return randomInt(random, ListDriver.MIN, ListDriver.MAX);
    }

    /**
     * Generate a Linked List of random integers
     *
     * @param n number of integers to generate
     */
    public static LinkedList generateList(Random random, int n)
    {
        LinkedList ll = new LinkedList();

        for (int i = 0; i < n; ++i) {
            ll.add(randomInt(random));
        }

        return ll;
    }

    public static void main(String[] args)
    {
        //----------------------------------------------------------------------
        // If a seed was passed from the command line,
        // parse it. Otherwise stick with the default seed
        //----------------------------------------------------------------------
        Random random = new Random();

        try {
            int seed = Integer.parseInt(args[0]);
            random.setSeed(seed);
        }
        catch (NumberFormatException | ArrayIndexOutOfBoundsException _exc) {
            // Do nothing... the default seed is fine
        }

        //----------------------------------------------------------------------
        // If a node count was passed from the command line,
        // parse it.
        //----------------------------------------------------------------------
        int toGenerate = 0;

        try {
            toGenerate = Integer.parseInt(args[1]);
        }
        catch (NumberFormatException | ArrayIndexOutOfBoundsException _exc) {
            toGenerate = DEFAULT_NUM_NODES;
        }

        //----------------------------------------------------------------------
        // Output the "fancy" program heading
        //----------------------------------------------------------------------
        System.out.println(projectHeading(PROGRAM_HEADING, W_WIDTH));

        //----------------------------------------------------------------------
        // Create and output a Linked List with random integers
        //----------------------------------------------------------------------
        LinkedList randomInts = generateList(random, toGenerate);
        System.out.println(randomInts);

        //----------------------------------------------------------------------
        // Create a copy of randomInts and add an "extra" number.
        //----------------------------------------------------------------------
        LinkedList randomCopy = randomInts.clone();
        randomCopy.add(337);

        //----------------------------------------------------------------------
        // Output the original list (again) after outputting the copy
        //----------------------------------------------------------------------
        final String starLine = horizontalLine('*', W_WIDTH);

        System.out.println(starLine);
        System.out.println(randomCopy);
        System.out.println(starLine);
        System.out.println(randomInts);

        //----------------------------------------------------------------------
        // Demonstrate what iterators allow us to do
        //----------------------------------------------------------------------
        LinkedList.Iterator it = randomInts.iterator();

        System.out.println("[");
        while (it.hasNext()) {
            System.out.printf("  %4d%n", it.next());
        }
        System.out.println("[");
    }
}
