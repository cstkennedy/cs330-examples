package edu.odu.cs.cs330.examples;

import java.io.IOException;
import java.util.Scanner;

public class KeyboardStrategy implements Strategy
{
    /**
     * Message used to prompt a human player for a move.
     */
    private static final String PROMPT_MSG = "Enter your desired move (1-9): ";

    private String _name;

    public KeyboardStrategy(String name)
    {
        this._name = name;
    }

    /**
     * Retrieve the next move.
     *
     * @return board cell id representing the selected move
     *
     * @throws IOException if the move can not be retreived from the player.
     */
    public int nextMove()
    {
        Scanner scnr = new Scanner(System.in);

        int choice;

        System.out.print(this._name + ", " + KeyboardStrategy.PROMPT_MSG);
        choice = scnr.nextInt();

        return choice;
    }
}
