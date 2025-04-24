import java.util.List;
import java.util.Arrays;
import java.util.Iterator;
import java.util.Vector;

public class SearchDemo
{


    /**
     * Search for a name
     *
     * @tparam Iterator type of iterator (dependent on data structure)
     *
     * @param start where to begin the search
     * @param end where to end the search
     * @param thingToFind name to locate
     *
     * @returns postion of a match or end if no match was found
     */
    public static String findName(Iterable<String> collection, final String thingToFind)
    {
        Iterator<String> searchIt = collection.iterator();

        while (searchIt.hasNext()) {
            // Look at the current name (*searchIt) and
            // compare to the name to find (thingToFind)
            String nextName = searchIt.next();
            if (nextName == thingToFind) {
                return nextName;
            }
        }

        return null;
    }

    public static void main(String... args)
    {
        System.out.println("--------Start Example--------");

        List<String> names = new Vector<String>();
        names.addAll(Arrays.asList("Thomas", "Jay", "Steve", "Janet", "Ravi"));

        System.out.println("---------While Loop Example----------");

        Iterator<String> it = names.iterator();

        while (it.hasNext()) {
            // Dereference and iterate
            System.out.printf("  - %s%n", it.next());
        }

        System.out.println("--------For-each Loop Example--------");
        for (final String n : names) {
            System.out.printf("  - %s%n", n);
        }

        //----------------------------------------------------------------------
        // Add a name - add demo
        //----------------------------------------------------------------------
        System.out.println("-------------Add a Name--------------");
        String newName = "Hill";
        names.add(newName);

        for (final String n : names) {
            System.out.printf("  - %s%n", n);
        }

        //----------------------------------------------------------------------
        // Search for a name
        //----------------------------------------------------------------------
        System.out.println("----------Search for a Name----------");
        final String thingToFind = "Thomas";

        // Original non-function search
        Iterator<String> searchIt = names.iterator();

        while (searchIt.hasNext()) {
            // Look at the current name (*searchIt) and
            // compare to the name to find (thingToFind)
            String nextName = searchIt.next();
            if (nextName == thingToFind) {
                System.out.println("Found a Match");
            }
        }

        final String foundStr = findName(names, "Andrey");

        if (foundStr == null) {
            System.out.println("No match was found");
        }
        else {
            System.out.println("Found a Match");
        }

        final boolean wasFound = names.contains("Steve");
        if (wasFound) {
            System.out.println("Found a Match");
        }
        else {
            System.out.println("No match was found");
        }
    }
}
