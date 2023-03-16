package edu.odu.cs.cs330.examples.guithread.driver;

import java.awt.BorderLayout;
import java.awt.FlowLayout;
import java.awt.Container;
import java.awt.event.ActionEvent;
import javax.swing.*;

import java.util.Arrays;
import java.util.List;
import java.util.ArrayList;

import edu.odu.cs.cs350.examples.numbers.PrimeGenerator;
import edu.odu.cs.cs330.examples.guithread.driver.PrimeOutputPanel;
import edu.odu.cs.cs330.examples.guithread.driver.PrimeGuiThread.PrimeInputPanel;

/**
 * This worker class contains all the logic for generating primes.
 * <p>
 * It is launched within a Thread--note that it implements Runnable
 */
public class PrimeWorker implements Runnable
{
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

    private PrimeInputPanel inputPanel;
    private PrimeOutputPanel outputPanel;
    private List<Integer> primeMasterList;

    /**
     * Construct a new Worker Instance.
     *
     * @param numPrimes desired number of primes to
     * generate
     */
    public PrimeWorker(int numPrimes, PrimeInputPanel inputPanel,
                       PrimeOutputPanel outputPanel,
                       List<Integer> primeMasterList)
    {
        this.primeMasterList = primeMasterList;
        primeGenerator = new PrimeGenerator(primeMasterList);

        // a negative toGenerate means that we have more primes than were
        // requested.
        toGenerate     = numPrimes - primeGenerator.numberOfPrimes();
        stop           = false;

        this.inputPanel = inputPanel;
        this.outputPanel = outputPanel;
    }

    /**
     * Perform the prime number generation.
     */
    @Override
    public void run()
    {
        for (int _i = 0; _i < toGenerate && !stop; ++_i) {
            primeGenerator.next();
        }

        this.outputPanel.renderResults(
            this.stop,
            this.primeGenerator.numberOfPrimes(),
            this.primeGenerator.getLast(),
            this.primeGenerator.toString()
        );

        this.inputPanel.toggleButtons();

        if (primeGenerator.numberOfPrimes() > primeMasterList.size()) {
            primeMasterList.clear();
            primeMasterList.addAll(primeGenerator.getPrimes());
        }

    }

    /**
     * Halt the prime number generation.
     */
    public void halt()
    {
        stop = true;
    }
}
