package edu.odu.cs.cs330.examples.tictactoe;

import java.io.IOException;

/**
 * Orchestrates a single match of Tic-Tac-Toe.
 */
public class Game
{
    private Board board;
    private Referee ref;

    private Player player1;
    private Player player2;

    private Player winner;
    private Player loser;

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
        this.board = new Board();
        this.ref = new Referee(this.board);

        this.player1 = p1;
        this.player2 = p2;
        this.winner = null;
        this.loser = null;
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
            if (this.playTurn(this.player1, 'X')) {
                break;
            }

            if (this.playTurn(this.player1, 'O')) {
                break;
            }
        }

        System.out.println(this.getBoard());

        if (this.endedWithWin()) {
            System.out.printf("Congratulations %s!%n", this.getWinner());
        }
    }

    /**
     * Play one half-round of Tic-Tac-Toe (i.e., one player's turn).
     *
     * @return true if the game ended during the turn (or at the start)
     *
     * @throws IOException if there is an error reading the selected move
     */
    public boolean playTurn(Player player, char symbol)
        throws IOException
    {
        if (board.isFull()) {
            return true;
        }

        System.out.println(board);
        int move = player.nextMove();

        while (this.ref.saysMoveIsNotValid(move)) {
            move = player.nextMove();
        }
        this.board.setCell(move, symbol);

        // Player won
        if(ref.checkForWin()) {
            if (symbol == 'X') {
                // Plater 1 won
                this.winner = player1;
                this.loser = player2;
            }
            else {
                // Player 2 won
                this.winner = player2;
                this.loser = player1;
            }
            return true;
        }

        return false;
    }

    public Player getPlayer1()
    {
        return this.player1;
    }

    public Player getPlayer2()
    {
        return this.player2;
    }

    public Player getWinner()
    {
        return this.winner;
    }

    public Player getLoser()
    {
        return this.loser;
    }

    public boolean endedWithWin()
    {
        return (this.winner != null);
    }

    public boolean endedWithStalemate()
    {
        return this.board.isFull()
            && (this.winner == null);
    }

    public boolean isOver()
    {
        return this.endedWithWin()
            || this.endedWithStalemate();
    }

    public boolean isNotOver()
    {
        return !this.isOver();
    }

    public Board getBoard()
    {
        return this.board;
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
            this.player1 = null;
            this.player2 = null;
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
}
