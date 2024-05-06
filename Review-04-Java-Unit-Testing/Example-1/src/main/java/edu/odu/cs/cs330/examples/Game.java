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
    public Game(Player p1, Player p2)
    {
        player1 = p1;
        player2 = p2;

        board = new Board();
        ref = new Referee(this.board);

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
        // The game ended already - assert could be used
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

};
