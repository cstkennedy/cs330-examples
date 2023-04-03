package edu.odu.cs.cs330.examples.guithread.driver;

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
     * An instance of Bruteforce Prime Number Generator.
     */
    private PrimeGenerator primeGenerator;

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

    /**
     * Construct a new Worker Instance.
     *
     * @param numPrimes desired number of primes to
     * generate
     */
    public PrimeWorker(int numPrimes,
                       U inputPanel,
                       T outputPanel)
    {
        this.reset(numPrimes);

        this.inputPanel = inputPanel;
        this.outputPanel = outputPanel;
    }

    /**
     * Set the worker to a state where stop is false with a new number of
     * primes to generate.
     *
     * @param numPrimes number of primes desired
     */
    public void reset(int numPrimes)
    {
        primeGenerator = new PrimeGenerator();

        // a negative toGenerate means that we have more primes than were
        // requested.
        toGenerate     = numPrimes - 2;
        stop           = false;
    }

    /**
     * Perform the prime number generation.
     */
    @Override
    public void run()
    {
        final long startTime = System.nanoTime();

        for (int _i = 0; _i < toGenerate && !stop; ++_i) {
            primeGenerator.next();
        }

        final long endTime = System.nanoTime();
        final double runTimeInSec = (endTime - startTime) / MICRO_SEC_TO_SEC;

        this.outputPanel.renderResults(
            this.stop,
            this.primeGenerator.numberOfPrimes(),
            this.primeGenerator.getLast(),
            this.primeGenerator.toString(),
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
