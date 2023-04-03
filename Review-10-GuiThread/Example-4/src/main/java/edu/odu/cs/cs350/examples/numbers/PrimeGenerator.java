package edu.odu.cs.cs350.examples.numbers;

import java.util.List;
import java.util.ArrayList;
import java.util.Iterator;

/**
 * This is a Brute Force Prime Number Generator. It is intentionally
 * inefficient.
 * <p>
 * There exist far more appropriate methods to identify prime numbers.
 */
public class PrimeGenerator implements Iterable<Integer>, Cloneable {
    /**
     * Ordered list of known primes.
     */
    private List<Integer> primes; // Inheritance & Well-defined interfaces

    /**
     * Initialize the Prime Number Generator with the first two known
     * primes--i.e., 2 and 3.
     */
    public PrimeGenerator()
    {
        primes  = new ArrayList<Integer>();

        primes.add(Integer.valueOf(2));
        primes.add(Integer.valueOf(3));
    }

    /**
     * Initialize the Prime Number Generator with a supplied list of known
     * primes.
     *
     * @param knownPrimes starting list of already generated primes (assumed to
     * be valid)
     */
    public PrimeGenerator(List<Integer> knownPrimes)
    {
        primes = new ArrayList(knownPrimes);
    }

    @Override
    public PrimeGenerator clone()
    {
        return new PrimeGenerator(this.primes);
    }

    /**
     * Check a number against list of known primes.
     *
     * @param n number to check
     *
     * @return true if the number is prime and false otherwise
     */
    private boolean isNumberPrime(int n)
    {
        // While the list of primes has not yet been exhausted
        // Check for divisibility by the next element--i.e.,
        // if nextPrime %p == 0 for any p, discard nextPrime
        /*
        for (int p : this.primes) {
            // Is the number prime?
            if (n % p == 0) {
                return false;
            }
        }

        return true;
        */
        /*
        final long numZeroRemainders = this.primes.parallelStream()
            .mapToInt(Integer::intValue)
            .takeWhile((int p) -> p <= Math.sqrt(n))
            .map(
                (int p) -> {
                    return n % p;
                }
            )
            .filter(
                (int remainder) -> {
                    return remainder == 0;
                }
            )
            .count();
        */
        final long numZeroRemainders = this.primes.parallelStream()
            .mapToInt(Integer::intValue)
            .takeWhile((int p) -> p <= Math.sqrt(n))
            .map((int p) -> n % p)
            .filter((int remainder) -> remainder == 0)
            .count();

        return numZeroRemainders == 0;
    }

    /**
     * Generate the next prime number.
     */
    public void next()
    {
        // prime from which to start calculations
        int nextPrime = getLast();

        // true once a prime number has been identified
        boolean prime = false;

        // halt when a prime number has been identified
        while (!prime) {
            // Guess the next prime
            nextPrime += 2;
            prime = isNumberPrime(nextPrime);
        }

        // record the new prime number
        primes.add(Integer.valueOf(nextPrime));
    }

    /**
     * Generate the next round of prime numbers.
     *
     * @param toGenerate number of primes to generate.
     */
    public void nextFew(int toGenerate)
    {
        for (int i = 0; i < toGenerate; i++) {
            this.next();
        }
    }

    /**
     * Get the last (most recently generated) prime.
     *
     * @return largest known prime as an int
     */
    public int getLast()
    {
        return primes.get(primes.size() - 1).intValue();
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
     * Return an iterator to the first known prime number.
     *
     * @return Iterator over all Integer prime numbers
     */
    @Override
    public Iterator<Integer> iterator()
    {
        return primes.iterator();
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

    /**
     * Compare two generators for equivalance based only on the number of
     * primes generated.
     */
    @Override
    public boolean equals(Object rhs)
    {
        if (!(rhs instanceof PrimeGenerator)) {
            return false;
        }

        PrimeGenerator rhsGen = (PrimeGenerator) rhs;

        return this.numberOfPrimes() == rhsGen.numberOfPrimes();
    }

    /**
     * Compute a hashcode (use the same attributes used by equals).
     */
    @Override
    public int hashCode()
    {
        return this.numberOfPrimes() * 4;
    }

    /**
     * List all generated primes (no specific formatting specified).
     */
    @Override
    public String toString()
    {
        /*
        StringBuilder bld = new StringBuilder();

        bld.append(primes.get(0));
        for (int i = 1; i < this.numberOfPrimes(); i++) {
            bld.append(", ");
            bld.append(primes.get(i));
        }

        return bld.toString();
        */

        return this.primes
            .stream()
            .map(p -> p.toString())
            .collect(java.util.stream.Collectors.joining("\n", "", ""));
    }
}
