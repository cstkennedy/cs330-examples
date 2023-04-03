package edu.odu.cs.cs330.examples.guithread.driver;

import java.awt.FlowLayout;
import java.awt.event.ActionListener;
import javax.swing.*;

public class PrimeInputPanel extends JPanel implements TraitControls
{
    private JTextField  toGenField;   ///< Input Field - # primes to generate
    private JLabel      toGenLabel;   ///< Label - # primes to generate

    private JButton     startButton;  ///< Control - start generation
    private JButton     stopButton;   ///< Halt - stop generation

    public PrimeInputPanel(ActionListener startListener, ActionListener stopListener)
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
        startButton.addActionListener(startListener);
        stopButton.addActionListener(stopListener);

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

