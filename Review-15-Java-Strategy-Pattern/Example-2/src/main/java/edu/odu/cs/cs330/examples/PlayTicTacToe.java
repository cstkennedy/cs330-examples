package edu.odu.cs.cs330.examples;

import java.io.IOException;

/**
 * The wrapper for main function.
 */
public class PlayTicTacToe
{
    /**
     * This program accepts no command line arguments.
     *
     * @param args not used
     */
    public static void main(String[] args)
        throws IOException
    {
        Game game = Game.builder()
            .withHumanPlayer("Thomas")
            .withHumanPlayer("Jay")
            .build();

        game.playMatch();
    }
}
