package edu.odu.cs.cs330.examples;

import java.util.Random;

/**
 * Coin Flip Task.
 */
public class FlipTask
    implements Runnable
{
    /**
     * Number of coin flips to simulate.
     */
    private long numTrials;

    /**
     * Number of heads flips.
     */
    private long numHeads;

    /**
     * Number of tails flips.
     */
    private long numTails;

    /**
     * Construct a new Task.
     *
     * @param nTrials desired number of coin flips
     */
    public FlipTask(long nTrials)
    {
        this.numTrials = nTrials;
        this.numHeads  = 0;
        this.numTails  = 0;
    }

    /**
     * Simulate coin flips.
     */
    public void run()
    {
        double rndDouble = 0;

        Random rGen = new Random();

        for (int i = 0; i < this.numTrials; i++) {
            //rndDouble = Math.random(); // This is why - synchronization
            rndDouble = rGen.nextDouble();

            if (rndDouble < 0.5) {
                numHeads++;
            }
            else {
                numTails++;
            }
        }
    }

    /**
     * Get the number of results that were heads.
     *
     * @return number of heads results
     */
    public long numberHeads()
    {
        return this.numHeads;
    }

    /**
     * Get the number of results that were tails.
     *
     * @return number of tails results
     */
    public long numberTails()
    {
        return this.numTails;
    }

    /**
     * Get the number number of Trials that
     * were set to run.
     *
     * @return number of trials
     */
    public long numberTrials()
    {
        return this.numTrials;
    }

    /**
     * Generate a String summarizing the coin flip results.
     */
    @Override
    public String toString()
    {
        return String.format("# Heads: %6d (%6.4f) / # Tails %6d (%6.4f)%n",
                             this.numHeads,
                             (1.0 * this.numHeads / this.numTrials),
                             this.numTails,
                             (1.0 * this.numTails / this.numTrials));
    }
}