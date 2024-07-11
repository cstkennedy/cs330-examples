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

        // TODO: symbols are actually Game specific. Goal... move out of Player
        player1.setSymbol('X');
        player2.setSymbol('O');
    }

    /**
     * Play one round of Tic-Tac-Toe.
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

        int winnerId = 0;

        System.out.println(board);
        roundTurn(player1);

        // The game is over
        if (board.isFull()) {
            System.out.println(board);
            return true;
        }

        winnerId = ref.checkForWin();

        if (winnerId == 1) {
            winner = player1;
            return true;
        }

        System.out.println(board);
        roundTurn(player2);

        // Final board
        System.out.println(board);

        winnerId = ref.checkForWin();

        if (winnerId == 2) {
            winner = player2;
            return true;
        }

        return false;
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
            //return Player::referenceCylon;
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
     * Get a player move, and update the board.
     */
    private boolean roundTurn(Player player)
        throws IOException
    {
        // TODO: Goal... move this into player construction
        Strategy theStrategy = new KeyboardStrategy(player.getName());

        int   move;
        char  sym;

        move = player.nextMove(theStrategy);
        sym  = player.getSymbol();

        //while (board.getCell(move) != 'X' && board.getCell(move) != 'O') {
        while (!ref.selectedCellIsEmpty(move)) {
            move = player.nextMove(theStrategy);
            sym  = player.getSymbol();
        }
        board.setCell(move, sym);

        // @todo add move validation
        return true;
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
