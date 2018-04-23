package edu.odu.cs.cs330.examples.guithread.generator.prime;

import java.util.List;
import java.util.ArrayList;
import java.util.ListIterator;

/**
 * This is a Brute Force Prime Number Generator. It is intentionally inefficient.
 * <p>
 * There exist far more appropriate methods to identify prime numbers.
 * <p>
 * See Dr Wahab's course sites:
 * <p>
 *  - CS 772, http://www.cs.odu.edu/~cs772/fall14/lectures/Number%20Theory.htm#Primes
 * <p>
 *  - CS 472, http://www.cs.odu.edu/~cs472/fall11/lectures/Number_Theory.htm#Primes
 *
 */
public class BruteForce {
    /**
     * Ordered list of known primes.
     */
    private List<Integer> primes; // Inheritance & Well-defined interfaces

    /**
     * Add the first 2 known primes--i.e., 2 and 3.
     */
    public BruteForce()
    {
        primes  = new ArrayList<Integer>();

        //primes.add(new Integer(2));
        //primes.add(new Integer(3));

        primes.add(Integer.valueOf(2));
        primes.add(Integer.valueOf(3));
    }

    /**
     * Generate the next prime number.
     *
     * @return next prime number as an int
     */
    public int next()
    {
        // prime from which to start calculations
        int nextPrime = primes.get(primes.size() - 1).intValue();

        // true once a prime number has been identified
        boolean prime = false;

        // Iterate of all existing known prime numbers
        // halt when a prime number has been identified
        while (!prime) {
            // Start at the beginning of the primes list
            ListIterator<Integer> li = primes.listIterator();

            // Guess the next prime
            // Assume the number is not prime
            nextPrime += 2;
            prime = true;

            // While the list of primes has not yet been exhausted
            // Check for divisibility by the next element--i.e.,
            // if nextPrime %p == 0 for any p, discard nextPrime
            while (li.hasNext() && prime) {
                // Retrieve the next prime, p, from
                // the list of primes, primes
                int p = li.next().intValue();

                // Is the number prime?
                prime = (nextPrime % p != 0);
            }
        }

        // record the new prime number
        primes.add(Integer.valueOf(nextPrime));

        return nextPrime;
    }

    /**
     * Return a copy of all generated primes.
     *
     * @return List of all generated prime numbers
     *
     */
    public final List<Integer> getPrimes()
    {
        return new ArrayList<Integer>(primes);
    }

    /**
     * Indicate the number of primes that were generated.
     *
     * @return number of generated primes
     */
    public int numberOfPrimes()
    {
        return primes.size();
    }
}