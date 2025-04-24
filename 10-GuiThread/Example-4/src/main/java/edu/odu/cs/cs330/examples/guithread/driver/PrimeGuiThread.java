package edu.odu.cs.cs330.examples.guithread.driver;

import java.awt.BorderLayout;
import java.awt.Container;
import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import javax.swing.*;

/**
 * A simple GUI driver for generator.prime.BruteForce.
 * <p>
 * This Gui makes explicit use of a worker thread.
 * <p>
 * The interface remains responsive when a large number of
 * primes is generated--e.g., 100000.
 */
public class PrimeGuiThread extends JFrame
{
    private PrimeInputPanel  inputPanel;
    private PrimeOutputPanel outputPanel;

    /**
     * Worker Thread - Wrapper for Prime Generation.
     */
    private PrimeWorker worker;

    public class StartButtonListener implements ActionListener
    {
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

            PrimeGuiThread.this.worker.reset(numPrimes);
            new Thread(PrimeGuiThread.this.worker).start();
        }
    }

    public class StopButtonListener implements ActionListener
    {
        @Override
        public void actionPerformed(ActionEvent event)
        {
            /*
            if (PrimeGuiThread.this.worker != null) {
                PrimeGuiThread.this.worker.halt();
            }*/
            PrimeGuiThread.this.worker.halt();
        }
    }

    /**
     * The constructor for the GUI.
     */
    public PrimeGuiThread()
    {
        super("Prime Generator");
        super.setLocationRelativeTo(null);
        super.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);

        this.inputPanel = new PrimeInputPanel(
            new StartButtonListener(),
            new StopButtonListener()
        );
        this.outputPanel = new PrimeOutputPanel();

        this.worker = new PrimeWorker(2, this.inputPanel, this.outputPanel);

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
