package edu.odu.cs.cs330.examples.guithread.driver;

import java.util.List;

import edu.odu.cs.cs330.examples.guithread.generator.prime.BruteForce;

/**
 * A simple command line test driver for Prime Generator.
 */
public class TestPrimeGenerator {

    public static void main(String[] args)
    {
        /**
         * Prime number generator instance.
         */
        BruteForce         gen = new BruteForce();

        /**
         * List of generated primes.
         */
        List<Integer> primes;

        /**
         * Number of primes to generate.
         */
        int                numPrimes = 0;

        // Parse command line argument 1
        try {
            // Parse as an integer
            numPrimes = Integer.parseInt(args[0]);
        }
        catch (NumberFormatException e) {
            // If the argument could not be parsed,
            // default to 10
            numPrimes = 10;
        }
        catch (ArrayIndexOutOfBoundsException e) {
            // If no argument was supplied, default
            // to 10
            numPrimes = 10;
        }

        // The primes 2 and 3 are added automatically by the generator
        numPrimes -= 2;

        // Generate numPrimes prime numbers
        //System.out.format("Generating %d Prime Numbers\n", numPrimes);
        System.out.format("Generating %d Prime Numbers%n", numPrimes);

        for (int i = 0; i < numPrimes; i++) {
            //System.out.format("%3d: %10d", i, gen.next());
            //System.out.println();
            gen.next();
        }

        // Print the resulting list of primes
        System.out.println();
        //System.out.format("Prime Numbers Generated: \n", numPrimes); // oops
        System.out.println("Prime Numbers Generated:");

        // Retrieve the list of generated primes
        primes = gen.getPrimes();

        // Print each prime
        for (Integer i : primes) {
            System.out.println(i);
        }
    }
}