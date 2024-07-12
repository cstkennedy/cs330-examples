from typing import Tuple

CellPair = Tuple[int, str]
CellTriple = Tuple[CellPair, CellPair, CellPair]


class Board:
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

        self._the_board = ["1", "2", "3", "4", "5", "6", "7", "8", "9"]

    def get_cell(self, cell_id) -> str:
        """
        Retrieve the value stored in a selected Cell.

        Args:
            cell_id: numeric id representing the desired cell

        Returns:
            value stored in the Cell

        Precondition:
            (cell_id > 0 && cell_id < 10) -> @todo change to
        """

        # assert(cell_id > 0 and cell_id < 10)
        assert 0 < cell_id < 10

        return self._the_board[cell_id - 1]  # Testing caught the missing -1

    def set_cell(self, cell_id, new_value):
        """
        Set the value stored in a selected Cell.

        Args:
            cell_id: numeric id representing the desired cell
            new_value: replacement `CellValue`

        Precondition:
            (id > 0 && id < 10) &&
            (
                (new_value == 'X' || new_value == 'O') ||
                (new_value >= '1' && new_value <= '9')
            )
        """

        # assert (cell_id > 0 and cell_id < 10)
        assert 0 < cell_id < 10

        # Testing caught the missing -1
        self._the_board[cell_id - 1] = new_value

    def get_3_cells(self, cell1_id, cell2_id, cell3_id) -> CellTriple:
        """
        Retrieve the value stored in three selected Cells.

        Args:
            cell1_id: numeric id representing the 1st desired cell
            cell2_id: numeric id representing the 2nd desired cell
            cell3_id: numeric id representing the 3rd desired cell

        Returns:
            value stored in the Cell

        Raises:
            IndexError if

                !(cell1_id > 0 && cell1_id < 10) ||
                !(cell2_id > 0 && cell2_id < 10) ||
                !(cell3_id > 0 && cell3_id < 10)
        """

        #  assert (cell1_id > 0 and cell1_id < 10)
        #  assert (cell2_id > 0 and cell2_id < 10)
        #  assert (cell3_id > 0 and cell3_id < 10)
        #  assert (0 < cell1_id < 10)
        #  assert (0 < cell2_id < 10)
        #  assert (0 < cell3_id < 10)

        if not (0 < cell1_id < 10):
            raise IndexError("Cell 1 Index is not between 0 and 10, exclusive")

        if not (0 < cell2_id < 10):
            raise IndexError("Cell 2 Index is not between 0 and 10, exclusive")

        if not (0 < cell3_id < 10):
            raise IndexError("Cell 3 Index is not between 0 and 10, exclusive")

        cell1_id -= 1
        cell2_id -= 1
        cell3_id -= 1

        return (
            (cell1_id, self._the_board[cell1_id]),
            (cell2_id, self._the_board[cell2_id]),
            (cell3_id, self._the_board[cell3_id]),
        )

    def is_full(self) -> bool:
        """
        Return true if all 9 cells hold player symbols.

        _I added this during implementation of the Game ADT in C++_

        Returns:
            True if every cell in the board has either an 'X' or an 'O'
        """

        empty_cells = 0

        for cell in self._the_board:
            if cell.isdigit():
                empty_cells += 1

        # return (empty_cells == 9) # OOPs... good thing I tested
        return empty_cells == 0

    def __eq__(self, rhs):
        if not isinstance(rhs, self.__class__):
            return False

        return self._the_board == rhs._the_board

    def __str__(self):
        """
        Print the Board.
        E.g.,
          1|2|3
          4|5|6
          7|8|9
        """

        return "\n".join(
            [
                "|".join(self._the_board[0:3]),
                "|".join(self._the_board[3:6]),
                "|".join(self._the_board[6:9]),
            ]
        )
