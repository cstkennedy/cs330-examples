package edu.odu.cs.cs330.examples;

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
    /**
     * Wrapper for a two-tuple consisting of a cell index
     * and cell content.
     */
    public static class Pair {
        public int  first;
        public char second;

        public Pair(int i, char s)
        {
            this.first  = i;
            this.second = s;
        }

        @Override
        public boolean equals(Object rhs)
        {
            if (!(rhs instanceof Pair)) {
                return false;
            }

            Pair rhsP = (Pair) rhs;

            return this.first == rhsP.first
                && this.second == rhsP.second;
        }
    }

    private char[] theBoard;

    /**
     * Construct an empty gameboard.
     */
    public Board()
    {
        theBoard = new char[9];

        for (int i = 0; i < theBoard.length; i++) {
            theBoard[i] = (char) ((int) '0' + (i + 1));
        }
    }

    /**
     * Retrieve the value stored in a selected Cell.
     *
     * @param id numeric id representing the desired cell
     *
     * @return value stored in the Cell
     *
     * @pre (id > 0 && id < 10) -> @todo change to
     *     throws IllegalArgumentException
     */
    char getCell(int id)
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
    void setCell(int id, char newValue)
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
    Pair[] get3Cells(int cell1Id, int cell2Id, int cell3Id)
    {
        assert (cell1Id > 0 && cell1Id < 10);
        assert (cell2Id > 0 && cell2Id < 10);
        assert (cell3Id > 0 && cell3Id < 10);

        cell1Id -= 1;
        cell2Id -= 1;
        cell3Id -= 1;

        Pair[] trpl = {
            new Pair(cell1Id, theBoard[cell1Id]),
            new Pair(cell2Id, theBoard[cell2Id]),
            new Pair(cell3Id, theBoard[cell3Id])
        };

        return trpl;
    }

    /**
     * Return true if all 9 cells hold player symbols.
     *
     * _I added this during implementation of the Game ADT in C++_
     *
     * @return true if every cell in the board has either an 'X' or an 'O'
     */
    boolean isFull()
    {
        int emptyCells = 0;

        for (int i = 0; i < theBoard.length; i++) {
            if (Character.isDigit(theBoard[i])) {
                emptyCells++;
            }
        }
        //return (emptyCells == 9); // OOPs... good thing I tested
        return (emptyCells == 0);
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

        for (int i = 0; i < 9; i++) {
            if (this.theBoard[i] != rhsBrd.theBoard[i]) {
                return false;
            }
        }

        return true;
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
        StringBuilder bld = new StringBuilder();

        // row output
        for (int i = 0; i < 3; i++) {
            // column output
            for (int j = 0; j < 2; j++) {
                int idx = 3 * i; // row offset
                idx += j; //column offset

                bld.append(theBoard[idx] + "|");
            }

            bld.append(theBoard[(3 * i) + 2] + "\n");
        }

        return bld.toString();
    }
}
