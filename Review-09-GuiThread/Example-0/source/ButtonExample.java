import java.awt.*;
import java.awt.event.*;
import javax.swing.*;

/**
 * A GUI Program that extends the JFrame class
 * to generate a Window.
 */
public class ButtonExample extends JFrame {
    /**
     * Control - start generation
     */
    private JButton clickButton;

    /**
     * Constructor
     */
    public ButtonExample()
    {
        // Invoke the JFrame (base class) constructor
        super("Button Example");

        // Set action for upper right close button
        setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);

        Container cp = getContentPane();

        // Create a button
        clickButton = new JButton("Click Me!");

        /* Add click button Listener
         * This binds an action to the
         * button click event
         * <p>
         * This is an example of an immediate class.
         * <p>
         * I am specializing (defining) class and
         * instantiating an object.
         */
        clickButton.addActionListener(
            new ActionListener() {
                public void actionPerformed(ActionEvent e)
                {
                    displayMessage();
                }
            }
        );

        // Setup and add to the Main Container
        cp.setLayout(new BorderLayout());

        cp.add(clickButton, BorderLayout.CENTER);

        // Package Everything
        pack();
        setLocationRelativeTo(null);
    }

    /**
     * Display Message
     */
    void displayMessage()
    {
        // Generate a message (pop-up)
        // dialog window
        JOptionPane.showMessageDialog(
            null,
            "The Game",
            "The Game",
            JOptionPane.ERROR_MESSAGE
        );
    }

    /**
     * Main function
     */
    public static void main(String[] args)
    {
        new ButtonExample().setVisible(true);
    }
}