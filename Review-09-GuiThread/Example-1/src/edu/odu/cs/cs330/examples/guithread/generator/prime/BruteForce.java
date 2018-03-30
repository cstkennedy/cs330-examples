package edu.odu.cs.cs330.examples.guithread.generator.prime;

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
    private ArrayList<Integer> primes; ///< ordered list of known primes

    /**
     * Add the first 2 known primes--i.e., 2 and 3
     */
    public BruteForce()
    {
        primes  = new ArrayList<Integer>();
        
        primes.add(new Integer(2));
        primes.add(new Integer(3));
    }

    /**
     * Generate the next prime number
     *
     * @return next prime number as an int 
     */
    public int next()
    {
        // prime from which to start calculations
        int next_prime = primes.get(primes.size() - 1).intValue();             
        
        // true once a prime number has been identified
        boolean prime = false;         

        // Iterate of all existing known prime numbers
        // halt when a prime number has been identified
        while( !prime ){
            // Start at the beginning of the primes list
            ListIterator<Integer> li = primes.listIterator();

            // Guess the next prime
            // Assume the number is not prime
            next_prime += 2;
            prime = true;

            // While the list of primes has not yet been exhausted
            // Check for divisibility by the next element--i.e.,            
            // if next_prime %p == 0 for any p, discard next_prime
            while( li.hasNext() && prime  ){
                // Retrieve the next prime, p, from 
                // the list of primes, primes
                int p = li.next().intValue();         

                // Is the number prime?
                prime = ( next_prime % p != 0 );
            } 
        }

        // record the new prime number
        primes.add(new Integer(next_prime));

        return next_prime;
    }

    /**
     * Return a copy of all generated primes
     *
     * @return List of all generated prime numbers
     *
     */
    public final ArrayList<Integer> getPrimes()
    {
        return new ArrayList<Integer>(primes);
    }

    /**
     * Indicate the number of primes that were generated
     */
    public int numberOfPrimes()
    {
        return primes.size();
    }
}