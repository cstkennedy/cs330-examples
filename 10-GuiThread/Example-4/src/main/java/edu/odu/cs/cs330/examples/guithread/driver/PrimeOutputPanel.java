package edu.odu.cs.cs330.examples.guithread.driver;

import java.awt.FlowLayout;
import java.awt.BorderLayout;
import javax.swing.*;

public class PrimeOutputPanel extends JPanel implements TraitRenderResults
{
    private JPanel      summaryPanel; ///< Panel containing all output elements

    private JTextField  lastField;    ///< Output Field - Last--i.e., largest-- prime generated
    private JLabel      lastLabel;    ///< Label - Last--i.e., largest-- prime generated

    private JTextArea   logArea;      ///< Output - all generated prime numbers
    private JScrollPane logPane;      ///< Make logArea scrollable

    public PrimeOutputPanel()
    {
        setUpLogArea();
        setUpSummaryPanel();

        // Setup and add to the Main Container
        this.setLayout(new BorderLayout());

        this.add(summaryPanel, BorderLayout.SOUTH);
        this.add(logPane, BorderLayout.CENTER);
    }

    /**
     * Set up the scrollable log area/panel.
     */
    void setUpLogArea()
    {
        logArea = new JTextArea("", 10, 20);

        logArea.setEditable(false);
        logPane = new JScrollPane(logArea,
                                  JScrollPane.VERTICAL_SCROLLBAR_ALWAYS,
                                  JScrollPane.HORIZONTAL_SCROLLBAR_NEVER);
    }

    /**
     * Set up the Summary Panel.
     */
    void setUpSummaryPanel()
    {
        summaryPanel = new JPanel();

        lastField    = new JTextField(20);
        lastLabel    = new JLabel("Last Prime:");

        // Disable lastField --i.e., use exclusively for output
        lastField.setEnabled(false);

        summaryPanel.setLayout(new FlowLayout());

        summaryPanel.add(lastLabel);
        summaryPanel.add(lastField);
    }

    @Override
    public void renderResults(boolean stopped,
            int numberOfPrimes, int lastPrime,
            String completeOutput, double runTimeInSec)
    {
        // Update GUI elements
        lastField.setText("" + lastPrime);

        StringBuilder bld = new StringBuilder();

        bld.append(String.format("Completed in %.2f seconds%n", runTimeInSec));

        // If generation was stopped early--i.e., interrupted--
        // Prepend a message indicating the number of primes that
        // were generated.
        if (stopped) {
            bld.append("Prime Generation Halted\n");
            bld.append("# Generated: ");
            bld.append(numberOfPrimes);
            bld.append('\n');
        }

        bld.append(completeOutput);

        logArea.setText(bld.toString());
    }

    /**
     * Clear previous output.
     */
    @Override
    public void clear()
    {
        //logArea.setText(new String());
        //lastField.setText(new String());

        logArea.setText("");
        lastField.setText("");
    }
}

