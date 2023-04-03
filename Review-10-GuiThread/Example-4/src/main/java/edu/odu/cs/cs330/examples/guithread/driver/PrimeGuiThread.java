package edu.odu.cs.cs330.examples.guithread.driver;

import java.awt.BorderLayout;
import java.awt.FlowLayout;
import java.awt.Container;
import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import javax.swing.*;

import java.util.Arrays;
import java.util.List;
import java.util.ArrayList;

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

    public class StartButtonEvent implements ActionListener
    {
        public StartButtonEvent()
        {
        }

        @Override
        public void actionPerformed(ActionEvent event)
        {
            int numPrimes;

            try {
                numPrimes = Integer.parseInt(
                    PrimeGuiThread.this.inputPanel.getToGenerateField()
                );
            }
            catch (NumberFormatException exc) {
                numPrimes = 10;
                PrimeGuiThread.this.inputPanel.setToGenerateField(numPrimes);
            }

            PrimeGuiThread.this.outputPanel.clear();
            PrimeGuiThread.this.inputPanel.toggle();

            PrimeGuiThread.this.worker = new PrimeWorker(
                numPrimes,
                PrimeGuiThread.this.inputPanel,
                PrimeGuiThread.this.outputPanel
            );
            new Thread(PrimeGuiThread.this.worker).start();
        }
    }

    public class PrimeInputPanel extends JPanel implements TraitControls
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
                new StartButtonEvent()
            );

            stopButton.addActionListener(
                (ActionEvent e) -> {
                    /*
                    if (PrimeGuiThread.this.worker != null) {
                        PrimeGuiThread.this.worker.halt();
                    }*/
                    PrimeGuiThread.this.worker.halt();
                }
            );

            // Initialize button states
            startButton.setEnabled(true);
            stopButton.setEnabled(false);
        }

        @Override
        public void toggle()
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
         * Retrieve the text in the JTextField.
         */
        public String getToGenerateField()
        {
            return toGenField.getText();
        }

        /**
         * Set the text in the JTextField to a specified integer.
         */
        public void setToGenerateField(int number)
        {
            toGenField.setText("" + number);
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
        super.setLocation(null);
        super.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);

        this.inputPanel = new PrimeInputPanel();
        this.outputPanel = new PrimeOutputPanel();

        this.worker = new PrimeWorker(
            2,
            PrimeGuiThread.this.inputPanel,
            PrimeGuiThread.this.outputPanel
        );

        // Setup and add to the Main Container
        Container cp = super.getContentPane();
        cp.setLayout(new BorderLayout());
        cp.add(inputPanel, BorderLayout.NORTH);
        cp.add(outputPanel, BorderLayout.CENTER);

        // Package Everything
        super.pack();
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
