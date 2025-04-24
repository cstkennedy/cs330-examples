package edu.odu.cs.cs330.examples.tictactoe;

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
     * Determine whether a given move is invalid for the current board state.
     *
     * @param cellId cell position (1 indexed)
     *
     * @return true if the move is valid
     */
    public boolean saysMoveIsValid(int cellId)
    {
        return this.boardRef.cellIsEmpty(cellId);
    }

    /**
     * Determine whether a given move is invalid for the current board state.
     *
     * @param cellId cell position (1 indexed)
     *
     * @return true if the move is valid
     */
    public boolean saysMoveIsNotValid(int cellId)
    {
        return this.boardRef.cellIsNotEmpty(cellId);
    }

    /**
     * Check for a win condition.
     *
     * @return if a player has won (scored 3 in a row).
     */
    public boolean checkForWin()
    {
        return (
            Stream.of(
                // Horizontal
                Boolean.valueOf(boardRef.allThreeMatch(1, 2, 3)),
                Boolean.valueOf(boardRef.allThreeMatch(4, 5, 6)),
                Boolean.valueOf(boardRef.allThreeMatch(7, 8, 9)),
                // Vertical
                Boolean.valueOf(boardRef.allThreeMatch(1, 4, 7)),
                Boolean.valueOf(boardRef.allThreeMatch(2, 5, 8)),
                Boolean.valueOf(boardRef.allThreeMatch(3, 6, 9)),
                // Diagonal
                Boolean.valueOf(boardRef.allThreeMatch(1, 5, 9)),
                Boolean.valueOf(boardRef.allThreeMatch(7, 5, 3))
            )
            .filter(winFound -> winFound.booleanValue())
            .count() >= 1
        );
    }
}
