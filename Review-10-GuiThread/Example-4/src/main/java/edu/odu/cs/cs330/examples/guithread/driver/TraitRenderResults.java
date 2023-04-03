package edu.odu.cs.cs330.examples.guithread.driver;

public interface TraitRenderResults
{
    /**
     * Take the results of Prime Number Generation and display them to the user.
     *
     * @param stopped flag indicating whether generation was interrupted (true)
     * @param numberOfPrimes total number of primes generated
     * @param lastPrime final prime number generated
     * @param completeOutput full list of primes
     * @param runTimeInSec total time taken to generate primes in seconds
     */
    void renderResults(boolean stopped,
         int numberOfPrimes, int lastPrime,
         String completeOutput, double runTimeInSec);

    /**
     * Clear previous output.
     */
    void clear();
}
