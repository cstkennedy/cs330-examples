
public class EvenGen
{
    public static void main(String... args)
    {
        if (args.length < 2) {
            System.out.println(" Usage: ./even_gen [lower_bound] [upper_bound]");
            System.exit(1);
        }

        // Assume all args are well formed (i.e., can be parsed as integers).
        final int lowerBound = Integer.parseInt(args[0]);
        final int upperBound = Integer.parseInt(args[1]);

        // The core even output logic
        System.out.printf("Range [%d, %d]", lowerBound, upperBound);
        System.out.println();

        for (int nextEven = lowerBound; nextEven <= upperBound; nextEven++) {
            System.out.printf("%d%n", nextEven);
        }

    }
}
