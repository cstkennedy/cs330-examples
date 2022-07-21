
class Board(object):
    """
    This ADT represents the gameboard used in a round
    of standard tic-tac-toe (i.e., a 3 x 3 grid)
    <p>
    Each entry in the Board is referred to as a _Cell_.
    A Cell can be empty, where it stores a value in the range 1-9
    The digit represents the cell id and is used to update a Cell
    """

    def __init__(self):
        """
        Construct an empty gameboard.
        """

        self._theBoard = ['1', '2', '3', '4', '5', '6', '7', '8', '9']

    def getCell(self, id):
        """
        Retrieve the value stored in a selected Cell.

        @param id numeric id representing the desired cell

        @return value stored in the Cell

        @pre (id > 0 && id < 10) -> @todo change to
            throws IllegalArgumentException
        """

        assert(id > 0 and id < 10)

        return self._theBoard[id - 1]  # Testing caught the missing -1

    def setCell(self, id, newValue):
        """
        Set the value stored in a selected Cell.

        @param id numeric id representing the desired cell
        @param newValue replacement `CellValue`

        @pre (id > 0 && id < 10) &&
                (
                    (newValue == 'X' || newValue == 'O') ||
                    (newValue >= '1' && newValue <= '9')
                )
        """

        assert (id > 0 and id < 10)

        self._theBoard[id - 1] = newValue  # Testing caught the missing -1

    def get3Cells(self, cell1Id, cell2Id, cell3Id):
        """
        Retrieve the value stored in three selected Cells.

        @param cell1Id numeric id representing the 1st desired cell
        @param cell2Id numeric id representing the 2nd desired cell
        @param cell3Id numeric id representing the 3rd desired cell

        @return value stored in the Cell

        @pre (cell1Id > 0 && cell1Id < 10) &&
             (cell2Id > 0 && cell2Id < 10) &&
             (cell3Id > 0 && cell3Id < 10)
        """

        assert (cell1Id > 0 and cell1Id < 10)
        assert (cell2Id > 0 and cell2Id < 10)
        assert (cell3Id > 0 and cell3Id < 10)

        cell1Id -= 1
        cell2Id -= 1
        cell3Id -= 1

        return ((cell1Id, self._theBoard[cell1Id]),
                (cell2Id, self._theBoard[cell2Id]),
                (cell3Id, self._theBoard[cell3Id]))

    def isFull(self):
        """
        Return true if all 9 cells hold player symbols.

        _I added this during implementation of the Game ADT in C++_

        @return true if every cell in the board has either an 'X' or an 'O'
        """

        emptyCells = 0

        for cell in self._theBoard:
            if cell.isdigit():
                emptyCells += 1

        # return (emptyCells == 9) # OOPs... good thing I tested
        return (emptyCells == 0)

    def __eq__(self, rhs):

        if not isinstance(rhs, self.__class__):
            return False

        return self._theBoard == rhs._theBoard

    def __str__(self):
        """
        Print the Board.
        E.g.,
          1|2|3
          4|5|6
          7|8|9
        """

        return "\n".join(["|".join(self._theBoard[0:3]),
                          "|".join(self._theBoard[3:6]),
                          "|".join(self._theBoard[6:9])])
