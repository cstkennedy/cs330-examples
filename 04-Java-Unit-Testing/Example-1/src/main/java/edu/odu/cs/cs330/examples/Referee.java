package edu.odu.cs.cs330.examples;

/**
 * The Meta-player that checks game status
 * e.g., checks for wins, who won, if there is
 * a stalemate.
 *
 * It is an implementation detail that is not
 * exposed to the outside world.
 */
public class Referee {
    /**
     * Reference to the gameboard.
     */
    private final Board boardRef;

    /**
     * This constructor must never be called.
     */
    private Referee()
    {
        boardRef = null;
    }

    /**
     * Create the referee and allow access
     * to a board through a constant reference variable.
     *
     * @param b the board to monitor
     */
    public Referee(final Board b)
    {
        this.boardRef = b;
    }

    /**
     * Check for a win condition.
     *
     * @return 1 if player1, or 2 if player2 won.
     *     A 0 indicates neither won
     */
    public int checkForWin()
    {
        int winner = checkForHorizontalWin();

        if (winner != 0) {
            return winner;
        }

        winner = checkForVerticalWin();

        if (winner != 0) {
            return winner;
        }

        winner = checkForDiagonalWin();

        if (winner != 0) {
            return winner;
        }

        return 0;
    }

    /**
     * Determine whether a cell in the board has been selected
     * by a player.
     *
     * @param move player candidate move
     *
     * @return true if the cell is currently empty
     */
    public boolean selectedCellIsEmpty(int move)
    {
        return boardRef.getCell(move) != 'X' && boardRef.getCell(move) != 'O';
    }


    /**
     * Check each row of the board for three 'X' or three 'O'
     * characters.
     *
     * @return 1 if player 1 has won, 2 if player 2 has one, or 0 if
     *     no one has won
     */
    private int checkForHorizontalWin()
    {
        Board.Pair[] triple = boardRef.get3Cells(1, 2, 3);

        if (allThreeMatch(triple)) {
            // if they match, grab the 'X' or 'O'
            return playerNumFromSymbol(triple[0].second);
        }

        triple = boardRef.get3Cells(4, 5, 6);

        if (allThreeMatch(triple)) {
            // if they match, grab the 'X' or 'O'
            return playerNumFromSymbol(triple[0].second);
        }

        triple = boardRef.get3Cells(7, 8, 9);

        if (allThreeMatch(triple)) {
            // if they match, grab the 'X' or 'O'
            return playerNumFromSymbol(triple[0].second);
        }

        return 0;
    }

    /**
     * Check each column of the board for three 'X' or three 'O'
     * characters.
     *
     * @return 1 if player 1 has won, 2 if player 2 has one, or 0 if
     *     no one has won
     */
    private int checkForVerticalWin()
    {
        Board.Pair[] triple = boardRef.get3Cells(1, 4, 7);

        if (allThreeMatch(triple)) {
            // if they match, grab the 'X' or 'O'
            return playerNumFromSymbol(triple[0].second); // Oops mistake
        }

        triple = boardRef.get3Cells(2, 5, 8);

        if (allThreeMatch(triple)) {
            // if they match, grab the 'X' or 'O'
            return playerNumFromSymbol(triple[0].second);
        }

        triple = boardRef.get3Cells(3, 6, 9);

        if (allThreeMatch(triple)) {
            // if they match, grab the 'X' or 'O'
            return playerNumFromSymbol(triple[0].second);
        }

        return 0;
    }

    /**
     * Check the two diagonals of the board for three 'X' or three 'O'
     * characters.
     *
     * @return 1 if player 1 has won, 2 if player 2 has one, or 0 if
     *     no one has won
     */
    private int checkForDiagonalWin()
    {
        Board.Pair[] triple = boardRef.get3Cells(1, 5, 9);

        if (allThreeMatch(triple)) {
            // if they match, grab the 'X' or 'O'
            return playerNumFromSymbol(triple[0].second);
        }

        triple = boardRef.get3Cells(7, 5, 3);

        if (allThreeMatch(triple)) {
            // if they match, grab the 'X' or 'O'
            return playerNumFromSymbol(triple[0].second);
        }

        return 0;
    }

    /**
     * Check for three matching symbols in the Pair-Triple.
     *
     * @param triple set of three cells to check
     *
     * @return true if all three pairs contain the same symbol
     */
    private boolean allThreeMatch(Board.Pair[] triple)
    {
        char firstVal = triple[0].second; // std::pair first and second

        int numMatches = 0;

        for (int i = 0; i < triple.length; i++) {
            if (firstVal == triple[i].second) {
                numMatches++;
            }
        }

        return (numMatches == 3);
    }

    /**
     * Given an 'X' or an 'O' determine which player is using the symbol.
     *
     * @param sym symbol to check
     *
     * @return 1 for player 1 or 2 for player 2
     *
     * @pre sym == 'X' or 'O'
     */
    private int playerNumFromSymbol(char sym)
    {
        /*
        if (sym == 'X') {
            return 1;
        }
        return 2;
        */

        return (sym == 'X' ? 1 : 2);
    }
}