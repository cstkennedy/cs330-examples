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
            .withPlayer(new Player("Thomas"))
            .withPlayer(new Player("Jay"))
            .build();

        //while(!(game.getBoard().isFull())) {
        while (game.isNotOver()) {
            game.playRound();
        }

        System.out.println(game.getBoard());

        if (game.endedWithWin()) {
            System.out.printf("Congratulations %s!%n", game.getWinner());
        }
    }
}
