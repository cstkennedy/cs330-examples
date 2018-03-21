#include <cstdlib>
#include <stdlib.h>
#include <ctype.h>
#include <cassert>

#include "Board.h"

/**
 *
 */
Board::Board()
{
    for (int i = 0; i < theBoard.size(); i++) {
        theBoard[i] = '0' + (i+1);
    }
    
}

/**
 *
 */
Board::CellValue Board::getCell(int id) const
{
    assert(id > 0 && id < 10);

    return theBoard[id - 1]; // Testing caught the missing -1
}

/**
 *
 */
void Board::setCell(int id, CellValue newValue)
{
    assert(id > 0 && id < 10);

    theBoard[id - 1] = newValue; // Testing caught the missing -1
}

/**
 *
 */
Board::CellTriple Board::get3Cells(int cell1Id, int cell2Id, int cell3Id) const
{
    assert(cell1Id > 0 && cell1Id < 10);
    assert(cell2Id > 0 && cell2Id < 10);
    assert(cell3Id > 0 && cell3Id < 10);

    CellTriple trpl;

    cell1Id -= 1;
    cell2Id -= 1;
    cell3Id -= 1;

    trpl[0] = {cell1Id, theBoard[cell1Id]};
    trpl[1] = {cell2Id, theBoard[cell2Id]};
    trpl[2] = {cell3Id, theBoard[cell3Id]};

    return trpl;
}

/**
 *
 */
bool Board::isFull() const
{
    int emptyCells = 0;

    for (int i = 0; i < theBoard.size(); i++) {
        if(isdigit(theBoard[i])) {
            emptyCells++;
        }
    }

    //return (emptyCells == 9); // OOPs... good thing I tested with fake unit tests
    return (emptyCells == 0);
}

/**
 *
 */
void Board::display(std::ostream& outs) const
{
    // row output
    for (int i = 0; i < 3; i++) {
        // column output
        for (int j = 0; j < 2; j++) {
            int idx = 3 * i; // row offset
            idx += j; //column offset

            std::cout << theBoard[idx] << "|";
        }

        std::cout << theBoard[(3*i) + 2] << "\n";
    }
}

