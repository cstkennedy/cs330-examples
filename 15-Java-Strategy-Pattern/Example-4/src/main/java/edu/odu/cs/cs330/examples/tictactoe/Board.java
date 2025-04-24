package edu.odu.cs.cs330.examples.tictactoe;

import java.util.Arrays;
import java.nio.CharBuffer;

/**
 * This ADT represents the gameboard used in a round
 * of standard tic-tac-toe (i.e., a 3 x 3 grid)
 * <p>
 * Each entry in the Board is referred to as a _Cell_.
 * A Cell can be empty, where it stores a value in the range 1-9
 * The digit represents the cell id and is used to update a Cell
 *
 */
public class Board
{
    private char[] theBoard;

    /**
     * Construct an empty gameboard.
     */
    public Board()
    {
        theBoard = new char[9];

        for (int i = 0; i < theBoard.length; i++) {
            theBoard[i] = Character.forDigit((i + 1), 10);
        }
    }

    /**
     * Retrieve the value stored in a selected Cell.
     *
     * @param id numeric id representing the desired cell
     *
     * @return value stored in the Cell
     *
     * @throws IllegalArgumentException if (id <= 0 or id > 10)
     */
    public char getCell(int id)
    {
        //assert(id > 0 && id < 10);

        return theBoard[id - 1]; // Testing caught the missing -1
    }

    /**
     * Set the value stored in a selected Cell.
     *
     * @param id numeric id representing the desired cell
     * @param newValue replacement `CellValue`
     *
     * @pre (id > 0 && id < 10) &&
            (
                (newValue == 'X' || newValue == 'O') ||
                (newValue >= '1' && newValue <= '9')
            )
     */
    public void setCell(int id, char newValue)
    {
        assert (id > 0 && id < 10);

        theBoard[id - 1] = newValue; // Testing caught the missing -1
    }

    /**
     * Retrieve the value stored in three selected Cells.
     *
     * @param cell1Id numeric id representing the 1st desired cell
     * @param cell2Id numeric id representing the 2nd desired cell
     * @param cell3Id numeric id representing the 3rd desired cell
     *
     * @return value stored in the Cell
     *
     * @pre (cell1Id > 0 && cell1Id < 10) &&
     *      (cell2Id > 0 && cell2Id < 10) &&
     *      (cell3Id > 0 && cell3Id < 10)
     */
    public char[] get3Cells(int cell1Id, int cell2Id, int cell3Id)
    {
        return new char[] {
            this.getCell(cell1Id),
            this.getCell(cell2Id),
            this.getCell(cell3Id)
        };
    }

    /**
     * Check for three matching symbols in the Pair-Triple.
     *
     * @param cell1Id numeric id representing the 1st desired cell
     * @param cell2Id numeric id representing the 2nd desired cell
     * @param cell3Id numeric id representing the 3rd desired cell
     *
     * @return true if all three pairs contain the same symbol
     */
    public boolean allThreeMatch(int cell1Id, int cell2Id, int cell3Id)
    {
        final char[] triple = this.get3Cells(cell1Id, cell2Id, cell3Id);
        final char firstVal = triple[0];

        for (int i = 1; i < triple.length; i++) {
            if (firstVal != triple[i]) {
                return false;
            }
        }

        return true;
    }

    /**
     * Determine whether a cell in the board has been selected
     * by a player.
     *
     * @param cellId cell position (1 indexed)
     *
     * @return true if the cell is currently empty
     */
    public boolean cellIsEmpty(int cellId)
    {
        return this.getCell(cellId) != 'X'
            && this.getCell(cellId) != 'O';
    }

    /**
     * Determine whether a cell in the board has **not** been selected
     * by a player.
     *
     * @param cellId cell position (1 indexed)
     *
     * @return true if the cell is **not** currently empty
     */
    public boolean cellIsNotEmpty(int cellId)
    {
        return !this.cellIsEmpty(cellId);
    }

    /**
     * Return true if all 9 cells hold player symbols.
     *
     * @return true if every cell in the board has either an 'X' or an 'O'
     */
    boolean isFull()
    {
        // Thank you to... <https://stackoverflow.com/a/31649229>
        // Java's Array.stream does not like char[] :(
        final long emptyCells = CharBuffer.wrap(this.theBoard)
            .chars()
            .filter(cellEntry -> Character.isDigit(cellEntry))
            .count();

        return emptyCells == 0;
    }

    @Override
    public int hashCode()
    {
        return theBoard.hashCode();
    }

    @Override
    public boolean equals(Object rhsObj)
    {
        if (!(rhsObj instanceof Board)) {
            return false;
        }

        Board rhsBrd = (Board) rhsObj;

        return Arrays.equals(this.theBoard, rhsBrd.theBoard);
    }

    /**
     * Print the Board.
     * E.g.,
     *   1|2|3
     *   4|5|6
     *   7|8|9
     */
    @Override
    public String toString()
    {
        return String.join(
            System.lineSeparator(),
            String.format("%s|%s|%s", this.theBoard[0], this.theBoard[1], this.theBoard[2]),
            String.format("%s|%s|%s", this.theBoard[3], this.theBoard[4], this.theBoard[5]),
            String.format("%s|%s|%s", this.theBoard[6], this.theBoard[7], this.theBoard[8]),
            ""
        );
    }
}
