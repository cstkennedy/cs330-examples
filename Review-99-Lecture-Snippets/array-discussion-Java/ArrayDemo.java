

public class ArrayDemo
{

    /**
     * Maximum allowed array size
     */
    public static final int MAX_SIZE = 10;

    /**
     * 20 Dash Divider
     */
    public static final String DIVIDER = "--------------------\n";

    public static void main(String... args)
    {
        if (args.length != 1) {
            System.out.printf("Usage: java -jar ArrayDemo.jar demoType%n");
            System.out.println("  demoType: static or dynamic");

            System.exit(1);
        }

        String demoType = args[0];

        if (demoType.equals("static")) {
            staticArrayDemo();
        }
        else if (demoType.equals("dynamic")) {
            dynamicArrayDemo();
        }
        else {
            System.err.printf("\"%s\" is not a valid demo type", demoType);

            System.exit(2);
        }
    }

    /**
     * Statically allocated array demo - keep track of what is actually used
     */
    static void staticArrayDemo()
    {
        String[] names = new String[MAX_SIZE];
        int used = 0;

        names[used++] = "Thomas";
        names[used++] = "Jay";

        System.out.print(DIVIDER);
        for (int i = 0; i < used; i++) {
            System.out.println(names[i]);
        }

        System.out.print(DIVIDER);
        printArray(names);
    }

    /**
     * Dynamically allocated array demo
     */
    static void dynamicArrayDemo()
    {
        String[] names = new String[2];
        int size = names.length;

        names[0] = "Thomas Kennedy";
        names[1] = "Jay Morris";

        // "Resize" - This should be a separate function
        String[] tempArray = new String[size + 1];

        for (int i = 0; i < size; i++) {
            tempArray[i] = names[i];
        }
        names = tempArray;
        // End "Resize"

        // Add - should be a separate function (push_back)
        names[2] = "Steve Zeil";
        size = 3;
        // End add

        System.out.print(DIVIDER);
        for (int i = 0; i < names.length; i++) {
            System.out.println(names[i]);
        }

        System.out.print(DIVIDER);
        printArray(names);
    }

    /**
     * Print an array using the usual index based for loop.
     *
     * @param toPrint the array to print
     * @param length size of the array (number of elements)
     */
    static void printArray(final String[] toPrint)
    {
        for (final String name : toPrint) {
            System.out.println(name);
        }
    }
}


