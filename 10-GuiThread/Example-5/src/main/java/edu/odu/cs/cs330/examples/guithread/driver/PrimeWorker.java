package edu.odu.cs.cs330.examples.guithread.driver;

import java.util.List;
import java.util.ArrayList;

import edu.odu.cs.cs350.examples.numbers.PrimeGenerator;

/**
 * This worker class contains all the logic for generating primes.
 * <p>
 * It is launched within a Thread--note that it implements Runnable
 */
public class PrimeWorker<T extends TraitRenderResults,
                         U extends TraitControls> implements Runnable
{
    /**
     * Number of microseconds in one second.
     */
    public static final double MICRO_SEC_TO_SEC = 1_000_000_000.0;

    /**
     * The number of primes to be generated.
     */
    private int toGenerate;

    /**
     * Flag that is set to false when stopButton has been clicked (and
     * generation needs to be stopped).
     */
    private boolean stop;

    private U inputPanel;
    private T outputPanel;

    private List<Integer> alreadyKnownPrimes;

    /**
     * Construct a new Worker Instance.
     *
     * @param numPrimes desired number of primes to
     * generate
     */
    public PrimeWorker(
        int numPrimes,
        U inputPanel,
        T outputPanel
    )
    {
        this.alreadyKnownPrimes = new ArrayList<>();
        this.reset(numPrimes);

        this.inputPanel = inputPanel;
        this.outputPanel = outputPanel;
    }

    /**
     * Set the worker to a state where stop is false with a new number of
     * primes to generate.
     *
     * @param numPrimes number of requested primes
     */
    public void reset(int numPrimes)
    {
        toGenerate     = numPrimes;
        stop           = false;
    }

    /**
     * Get the time elapsed.
     *
     * @param start start time in nanoseconds
     *
     * @return elapsed time in seconds
     */
    private static double getTimeSince(final long startTime)
    {
        final long endTime = System.nanoTime();
        return (endTime - startTime) / MICRO_SEC_TO_SEC;
    }

    /**
     * Perform the prime number generation.
     */
    @Override
    public void run()
    {
        PrimeGenerator primeGenerator;
        final int numPrimes = toGenerate;

        if (numPrimes <= this.alreadyKnownPrimes.size()) {
            primeGenerator = new PrimeGenerator(this.alreadyKnownPrimes);
        }
        else {
            primeGenerator = new PrimeGenerator();
        }

        final long startTime = System.nanoTime();
        while (primeGenerator.numberOfPrimes() < numPrimes && !stop) {
            primeGenerator.next();
        }
        final double runTimeInSec = PrimeWorker.getTimeSince(startTime);

        if (this.alreadyKnownPrimes.size() < primeGenerator.numberOfPrimes()) {
            this.alreadyKnownPrimes = primeGenerator.getPrimes();
        }

        this.outputPanel.renderResults(
            this.stop,
            this.alreadyKnownPrimes.size(),
            primeGenerator.getLast(),
            primeGenerator.toString(),
            runTimeInSec
        );

        this.inputPanel.toggle();
    }

    /**
     * Halt the prime number generation.
     */
    public void halt()
    {
        stop = true;
    }
}
