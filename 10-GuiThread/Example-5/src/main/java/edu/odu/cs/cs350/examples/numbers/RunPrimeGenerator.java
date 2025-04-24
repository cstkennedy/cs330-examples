package edu.odu.cs.cs350.examples.numbers;


/**
 * A simple command line test driver for Prime Generator.
 */
public class RunPrimeGenerator {
    /**
     * The main function for the command line prime number generator.
     */
    public static void main(String[] args)
    {
        int numPrimes = 0;

        try {
            numPrimes = Integer.parseInt(args[0]);
        }
        catch (NumberFormatException | ArrayIndexOutOfBoundsException ignore) {
            numPrimes = 10;
        }

        // The primes 2 and 3 are added automatically by the generator
        numPrimes -= 2;

        System.out.format("Generating %d Prime Numbers%n", numPrimes);
        System.out.println();

        PrimeGenerator gen = new PrimeGenerator();
        gen.nextFew(numPrimes);

        System.out.println("Prime Numbers Generated:");

        for (Integer i : gen) {
            System.out.println(i);
        }
    }
}
