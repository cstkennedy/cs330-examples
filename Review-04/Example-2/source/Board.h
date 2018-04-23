#ifndef BOARD_H_INCLUDED
#define BOARD_H_INCLUDED

#include <iostream>
#include <array>

/**
 * This ADT represents the gameboard used in a round
 * of standard tic-tac-toe (i.e., a 3 x 3 grid)
 * <p>
 * Each entry in the Board is referred to as a _Cell_.
 * A Cell can be empty, where it stores a value in the range 1-9
 * The digit represents the cell id and is used to update a Cell
 *
 */
class Board {
    public:
        /**
         * Value of one Cell
         */
        typedef char CellValue;

        /**
         * Collection of 3 `Board::Cell` ids and the corresponding values
         * <p>
         * If you like "Alice in Wonderland" the full documentation of std::array
         * is located at http://en.cppreference.com/w/cpp/container/array. Be careful,
         * it is easy to get lost in the rabbit hole
         */
        //typedef std::array<std::pair<int, Board::CellValue>, 3> CellTriple;
        using CellTriple = std::array<std::pair<int, Board::CellValue>, 3>;

    private:
        std::array<CellValue, 9> theBoard;

    public:
        /**
         * Construct an empty gameboard
         */
        Board();

        /**
         * Retrieve the value stored in a selected Cell
         *
         * @param id numeric id representing the desired cell
         *
         * @return value stored in the Cell
         *
         * @pre (id > 0 && id < 10)
         */
        CellValue getCell(int id) const;

        /**
         * Set the value stored in a selected Cell
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
        void setCell(int id, CellValue newValue);

        /**
         * Retrieve the value stored in three selected Cells
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
        CellTriple get3Cells(int cell1Id, int cell2Id, int cell3Id) const;

        /**
         * Return true if all 9 cells hold player symbols
         *
         * _I added this during implementation of the Game ADT_
         */
        bool isFull() const;

        /**
         * Print the Board
         * E.g., 
         *   1|2|3
         *   4|5|6
         *   7|8|9
         */
        void display(std::ostream& outs) const;
};

/**
 * Make output easier with the standard stream-insertion operator
 */
inline
std::ostream& operator<<(std::ostream& outs, const Board& prt)
{
    prt.display(outs);

    return outs;
}

#endif