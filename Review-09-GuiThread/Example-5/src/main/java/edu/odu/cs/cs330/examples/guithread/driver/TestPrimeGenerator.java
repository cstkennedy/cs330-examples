package edu.odu.cs.cs330.examples.guithread.driver;

import java.util.ArrayList;

import edu.odu.cs.cs330.examples.guithread.generator.prime.BruteForce;

/**
 * A simple command line test driver for Prime Generator
 */
public class TestPrimeGenerator {
    
    public static void main(String args[])
    {
        BruteForce         gen = new BruteForce(); // Prime number generator instance
        ArrayList<Integer> primes;                 // List of generated primes

        int                num_primes = 0;         // Number of primes to generate

        // Parse command line argument 1
        try {
            // Parse as an integer
            num_primes = Integer.parseInt(args[0]);
        }
        catch(NumberFormatException e) {
            // If the argument could not be parsed,
            // default to 10
            num_primes = 10;
        }
        catch(ArrayIndexOutOfBoundsException e) {
            // If no argument was supplied, default
            // to 10
            num_primes = 10;
        }

        // The primes 2 and 3 are added automatically by the generator
        num_primes -= 2;

        // Generate num_primes prime numbers
        //System.out.format("Generating %d Prime Numbers\n", num_primes);
        System.out.format("Generating %d Prime Numbers%n", num_primes);

        for(int i = 0; i < num_primes; i++) {
            //System.out.format("%3d: %10d", i, gen.next());
            //System.out.println();
            gen.next();
        }

        // Print the resulting list of primes
        System.out.println();
        //System.out.format("Prime Numbers Generated: \n", num_primes);
        //System.out.format("Prime Numbers Generated: \n"); // num_primes (typo)
        System.out.format("Prime Numbers Generated: %n");

        // Retrieve the list of generated primes
        primes = gen.getPrimes();

        // Print each prime
        for(Integer i : primes) {
            System.out.println(i);
        }
    }
}