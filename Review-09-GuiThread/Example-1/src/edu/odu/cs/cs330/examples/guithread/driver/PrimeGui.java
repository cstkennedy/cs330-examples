package edu.odu.cs.cs330.examples.guithread.driver;

import java.awt.*;
import java.awt.event.*;
import javax.swing.*;

import edu.odu.cs.cs330.examples.guithread.generator.prime.BruteForce;

/**
 * A simple GUI driver for generator.prime.BruteForce.
 * <p>
 * This Gui does not make explicit use of threads.
 * <p>
 * It appears to lockup when a large number of primes is
 * generated--e.g., 100000.
 */
public class PrimeGui extends JFrame {
    /**
     * This worker class contains all the logic for generating primes.
     * <p>
     * It is not a Thread.
     */
    private class PrimeWorker {
        // Data--i.e., computation--elements
        private BruteForce primeGenerator; ///< Instance of BruteForce prime # generator
        private int        toGenerate;     ///< Number of primes to generate
        private boolean    stop;           ///< Flag - set to false when stopButton is clicked

        /**
         * Construct a new Worker Instance.
         *
         * @param numPrimes desired number of primes to
         * generate
         */
        public PrimeWorker(int numPrimes)
        {
            primeGenerator = new BruteForce();

            toGenerate     = numPrimes;
            stop           = false;
        }

        /**
         * Perform the prime number generation.
         */
        public void run()
        {
            int prime = 2;

            for(int i = 0; i < toGenerate && !stop; i++) {
                prime = primeGenerator.next();                
            }

            // Update GUI elements
            lastField.setText("" + prime);

            StringBuilder bld = new StringBuilder();

            for (Integer i : primeGenerator.getPrimes()) {
                bld.append(i);
                bld.append("\n");
            }

            logArea.setText(bld.toString());
        }

        /**
         * Halt the prime number generation.
         */
        public void halt()
        {
            stop = true;
        }
    }

    // Interface Elements
    private JPanel      inputPanel;   ///< Panel containing all input elements
    private JPanel      summaryPanel; ///< Panel containing all output elements

    private JTextField  lastField;    ///< Output Field - Last--i.e., largest-- prime generated 
    private JLabel      lastLabel;    ///< Label - Last--i.e., largest-- prime generated 

    private JTextField  toGenField;   ///< Input Field - # primes to generate
    private JLabel      toGenLabel;   ///< Label - # primes to generate

    private JTextArea   logArea;      ///< Output - all generated prime numbers

    private JButton     startButton;  ///< Control - start generation
    private JButton     stopButton;   ///< Halt - stop generation

    // Data--i.e., computation--elements
    private PrimeWorker worker; ///< Worker Thread - Wrapper for Prime Generation

    public PrimeGui()
    {
        super("Prime Generator");
        setLocation(50, 75);
        setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);

        Container cp = getContentPane();

        // Initialize Interface Elements
        inputPanel   = new JPanel();
        summaryPanel = new JPanel();

        lastField    = new JTextField(20);
        lastLabel    = new JLabel("Last Prime:");

        toGenField   = new JTextField(10);
        toGenLabel   = new JLabel("# Primes:");

        // Disable lastField --i.e., use exclusively for output
        lastField.setEnabled(false);

        startButton  = new JButton("Start");
        stopButton   = new JButton("Stop");

        // Initialize Text Area
        logArea      = new JTextArea("", 10, 20);

        logArea.setEditable(false);
        JScrollPane logPane = new JScrollPane(
            logArea,
            JScrollPane.VERTICAL_SCROLLBAR_ALWAYS,
            JScrollPane.HORIZONTAL_SCROLLBAR_NEVER
        );

        // Set Panel Layouts
        inputPanel.setLayout(new FlowLayout());
        summaryPanel.setLayout(new FlowLayout());

        // Populate the Input Panel
        inputPanel.add(toGenLabel);
        inputPanel.add(toGenField);
        inputPanel.add(startButton);
        inputPanel.add(stopButton);

        // Populate the summary Panel
        summaryPanel.add(lastLabel);
        summaryPanel.add(lastField);

        //Add Action Listeners to the Buttons

        // Add start button Listener
        startButton.addActionListener(
            new ActionListener() {
                public void actionPerformed(ActionEvent e)
                {
                    int numPrimes;

                    try {
                        numPrimes = Integer.parseInt(toGenField.getText());
                    }
                    catch (NumberFormatException exc) {
                        numPrimes = 10;
                        toGenField.setText("" + numPrimes);
                    }

                    numPrimes -= 2;

                    clear();
                    toggleButtons();

                    worker = new PrimeWorker(numPrimes);
                    worker.run();

                    toggleButtons();
                }
            }
        );

        // Add stop button Listener
        stopButton.addActionListener(
            new ActionListener() {
                public void actionPerformed(ActionEvent e)
                {
                    if (worker != null) {
                        worker.halt();
                    }

                    toggleButtons();
                }
            }
        );

        // Setup and add to the Main Container
        cp.setLayout(new BorderLayout());

        cp.add(inputPanel, BorderLayout.NORTH);
        cp.add(summaryPanel, BorderLayout.SOUTH);

        cp.add(logPane, BorderLayout.CENTER);

        // Package Everything
        pack();

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

    /**
     * Clear previous output.
     */
    void clear()
    {
        logArea.setText(new String());
        lastField.setText(new String());
    }

    /**
     * The main function.
     *
     * @param args no command line arguments are used
     */
    public static void main(String[] args)
    {
        new PrimeGui().setVisible(true);
    }
}