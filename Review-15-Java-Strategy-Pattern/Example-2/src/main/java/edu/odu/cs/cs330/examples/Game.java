package edu.odu.cs.cs330.examples;

import java.io.IOException;

/**
 * Orchestrates a single match of Tic-Tac-Toe.
 */
public class Game
{
    private Board board;
    private Referee ref; // The referee for this game

    private Player player1;
    private Player player2;

    private Player winner;

    private int numMovesMade;

    // Disable Game Default Constructor
    private Game()
    {

    }

    /**
     * Construct a Game setting player 1 and player 2.
     *
     * @param p1 the 'X' player
     * @param p2 the 'O' player
     */
    private Game(Player p1, Player p2)
    {
        player1 = p1;
        player2 = p2;

        board = new Board();
        ref = new Referee(this.board);
    }

    /**
     * Play one match... until either the board is full or someone gets three
     * in a row.
     */
    public void playMatch()
        throws IOException
    {
        //while(!(this.getBoard().isFull())) {
        while (this.isNotOver()) {
            this.playRound();
        }

        System.out.println(this.getBoard());

        if (this.endedWithWin()) {
            System.out.printf("Congratulations %s!%n", this.getWinner());
        }
    }

    /**
     * Play one round of Tic-Tac-Toe (One sequence of player 1 and player 2).
     *
     * @return true if the game ended during the round
     *
     * @throws IOException if there is an error reading the selected move
     */
    public boolean playRound()
        throws IOException
    {
        if (board.isFull()) {
            return true;
        }

        System.out.println(board);
        this.performTurn(player1, 'X');

        // The game is over
        if (board.isFull()) {
            System.out.println(board);
            return true;
        }

        int winnerId = ref.checkForWin();

        if (winnerId == 1) {
            winner = player1;
            return true;
        }

        System.out.println(board);
        this.performTurn(player2, 'O');

        // Final board
        System.out.println(board);

        winnerId = ref.checkForWin();

        if (winnerId == 2) {
            winner = player2;
            return true;
        }

        return false;
    }

    /**
     * Get a player move... and update the board.
     */
    private void performTurn(Player player, char symbol)
        throws IOException
    {
        int move = player.nextMove();

        while (!ref.selectedCellIsEmpty(move)) {
            move = player.nextMove();
        }
        board.setCell(move, symbol);
    }

    public Player getPlayer1()
    {
        return player1;
    }

    public Player getPlayer2()
    {
        return player2;
    }

    public Player getWinner()
    {
        return winner;
    }

    public Player getLoser()
    {
        // @discussThisInLecture
        // Caught this bug during testing
        if (this.isNotOver()) {
            return null;
        }

        // Stalemate
        if (endedWithStalemate()) {
            return null;
        }

        // There was a win, figure out who lost
        if (winner.equals(player1)) {
            return player2;
        }

        return player1;
    }

    public boolean endedWithWin()
    {
        return (winner != null);
    }

    public boolean endedWithStalemate()
    {
        return (board.isFull()) && (winner == null);
    }

    public boolean isOver()
    {
        return (endedWithWin() || endedWithStalemate());
    }

    public boolean isNotOver()
    {
        return !isOver();
    }

    public Board getBoard()
    {
        return board;
    }

    /**
     * Orchestrator for Game construction.
     */
    public static class Builder
    {
        Player player1;
        Player player2;

        public Builder()
        {
            player1 = null;
            player2 = null;
        }

        /**
         * Add a player.
         *
         * @throws TBDException if both players have already been set.
         */
        public Builder withPlayer(final Player player)
        {
            if (this.player1 == null) {
                this.player1 = player;
            }
            else if (this.player2 == null) {
                this.player2 = player;
            }
            else {
                // throw appropriate exception
            }

            return this;
        }

        /**
         * Crate and add a human player. This handles setting up the Keyboard
         * input logic.
         *
         * @param name desired player name
         *
         * @throws TBDException if both players have already been set.
         */
        public Builder withHumanPlayer(String name)
        {
            return this.withPlayer(new Player(name, new KeyboardStrategy(name)));
        }

        /**
         * Actually create the Game.
         *
         * @throws TBDException if exactly two players have not been set.
         */
        public Game build()
        {
            if (this.player1 == null || this.player2 == null) {
                // Both players must be set
                // throw appropriate exception
            }

            return new Game(this.player1, this.player2);
        }

    }

    public static Game.Builder builder()
    {
        return new Game.Builder();
    }

};
