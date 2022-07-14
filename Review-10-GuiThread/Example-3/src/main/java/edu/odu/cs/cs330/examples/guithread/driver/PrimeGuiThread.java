package edu.odu.cs.cs330.examples.guithread.driver;

import java.awt.BorderLayout;
import java.awt.FlowLayout;
import java.awt.Container;
import java.awt.event.ActionEvent;
import javax.swing.*;

import edu.odu.cs.cs350.examples.numbers.PrimeGenerator;
import edu.odu.cs.cs330.examples.guithread.driver.PrimeOutputPanel;

/**
 * A simple GUI driver for generator.prime.BruteForce.
 * <p>
 * This Gui makes explicit use of a worker thread.
 * <p>
 * The interface remains responsive when a large number of
 * primes is generated--e.g., 100000.
 */
public class PrimeGuiThread extends JFrame {

    /**
     * This worker class contains all the logic for generating primes.
     * <p>
     * It is launched within a Thread--note that it implements Runnable
     */
    private class PrimeWorker implements Runnable
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

        /**
         * Construct a new Worker Instance.
         *
         * @param numPrimes desired number of primes to
         * generate
         */
        public PrimeWorker(int numPrimes)
        {
            primeGenerator = new PrimeGenerator();

            toGenerate     = numPrimes - 2; // The -2 is a Worker responsibility
            stop           = false;
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

            PrimeGuiThread.this.outputPanel.renderResults(
                this.stop,
                this.primeGenerator.numberOfPrimes(),
                this.primeGenerator.getLast(),
                this.primeGenerator.toString()
            );

            PrimeGuiThread.this.inputPanel.toggleButtons();
        }

        /**
         * Halt the prime number generation.
         */
        public void halt()
        {
            stop = true;
        }
    }

    private class PrimeInputPanel extends JPanel
    {
        private JTextField  toGenField;   ///< Input Field - # primes to generate
        private JLabel      toGenLabel;   ///< Label - # primes to generate

        private JButton     startButton;  ///< Control - start generation
        private JButton     stopButton;   ///< Halt - stop generation

        public PrimeInputPanel()
        {
            toGenField  = new JTextField(10);
            toGenLabel  = new JLabel("# Primes:");

            startButton = new JButton("Start");
            stopButton  = new JButton("Stop");

            this.setLayout(new FlowLayout());

            this.add(toGenLabel);
            this.add(toGenField);
            this.add(startButton);
            this.add(stopButton);

            // Add Action Listeners to the Buttons
            startButton.addActionListener(
                (ActionEvent e) -> {
                    int numPrimes;

                    try {
                        numPrimes = Integer.parseInt(toGenField.getText());
                    }
                    catch (NumberFormatException exc) {
                        numPrimes = 10;
                        toGenField.setText("" + numPrimes);
                    }

                    PrimeGuiThread.this.outputPanel.clear();
                    PrimeGuiThread.this.inputPanel.toggleButtons();

                    PrimeGuiThread.this.worker = new PrimeWorker(numPrimes);
                    new Thread(PrimeGuiThread.this.worker).start();
                }
            );

            stopButton.addActionListener(
                (ActionEvent e) -> {
                    if (worker != null) {
                        worker.halt();
                    }
                }
            );

            // Initialize button states
            startButton.setEnabled(true);
            stopButton.setEnabled(false);
        }

        /**
         * Toggle start button and stop button states.
         */
        void toggleButtons()
        {
            if (startButton.isEnabled()) {
                startButton.setEnabled(false);
                stopButton.setEnabled(true);
            }
            else {
                startButton.setEnabled(true);
                stopButton.setEnabled(false);
            }
        }
    }

    private PrimeInputPanel  inputPanel;
    private PrimeOutputPanel outputPanel;

    /**
     * Worker Thread - Wrapper for Prime Generation.
     */
    private PrimeWorker worker;

    /**
     * The constructor for the GUI.
     */
    public PrimeGuiThread()
    {
        super("Prime Generator");
        setLocation(50, 75);
        setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);

        Container cp = getContentPane();

        inputPanel = new PrimeInputPanel();
        outputPanel = new PrimeOutputPanel();

        // Setup and add to the Main Container
        cp.setLayout(new BorderLayout());
        cp.add(inputPanel, BorderLayout.NORTH);
        cp.add(outputPanel, BorderLayout.CENTER);

        // Package Everything
        pack();
    }

    /**
     * The main function.
     *
     * @param args no command line arguments are used
     */
    public static void main(String[] args)
    {
        new PrimeGuiThread().setVisible(true);
    }
}
