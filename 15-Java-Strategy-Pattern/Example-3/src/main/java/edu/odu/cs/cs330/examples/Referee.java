package edu.odu.cs.cs330.examples;

import java.util.stream.Stream;
import java.util.Optional;

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
     * Utility function to check whether a character is 'X' or 'O' (i.e., a
     * player symbol).
     */
    private static boolean isPossiblePlayerSymbol(final char theChar)
    {
        return theChar == 'X' || theChar == 'O';
    }

    /**
     * Check for a win condition.
     *
     * @return 'X' if player1, or 'O' if player2 won.
     *     Any other character indicates neither player won
     */
    public char checkForWin()
    {
        Optional<Board.Pair[]> match = Stream.of(
                // Horizontal
                boardRef.get3Cells(1, 2, 3),
                boardRef.get3Cells(4, 5, 6),
                boardRef.get3Cells(7, 8, 9),
                // Vertical
                boardRef.get3Cells(1, 4, 7),
                boardRef.get3Cells(2, 5, 8),
                boardRef.get3Cells(3, 6, 9),
                // Diagonal
                boardRef.get3Cells(1, 5, 9),
                boardRef.get3Cells(7, 5, 3)
            )
            .filter(triple -> allThreeMatch(triple))
            .findFirst();

        if (match.isPresent()) {
            return match.get()[0].second;
        }

        return '?';
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
}
