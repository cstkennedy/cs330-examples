VALID_SYMBOLS = ("X", "O")


class Board:
    """
    This ADT represents the gameboard used in a round
    of standard tic-tac-toe (i.e., a 3 x 3 grid)
    <p>
    Each entry in the Board is referred to as a _Cell_.
    A Cell can be empty, where it stores a value in the range 1-9
    The digit represents the cell id and is used to update a Cell
    """

    def __init__(self) -> None:
        """
        Construct an empty gameboard.
        """

        self._the_board = ["1", "2", "3", "4", "5", "6", "7", "8", "9"]

    def get_cell(self, cell_id: int) -> str:
        """
        Retrieve the value stored in a selected Cell.

        Args:
            cell_id: numeric id representing the desired cell

        Returns:
            value stored in the Cell

        Raises:
            IndexError if !(cell1_id > 0 && cell1_id < 10) ||

        """

        if not 0 < cell_id < 10:
            raise IndexError("Cell Index is not between 0 and 10, exclusive")

        return self._the_board[cell_id - 1]  # Testing caught the missing -1

    def set_cell(self, cell_id: int, new_value: str) -> None:
        """
        Set the value stored in a selected Cell.

        Args:
            cell_id: numeric id representing the desired cell
            new_value: replacement `CellValue`

        Raises:
            IndexError if !(cell1_id > 0 && cell1_id < 10) ||

            ValueError if new_value is not 'X' or 'O'
        """

        if not 0 < cell_id < 10:
            raise IndexError("Cell Index is not between 0 and 10, exclusive")

        if new_value not in VALID_SYMBOLS:
            raise ValueError(f"'{new_value}' is not 'X' or 'O'")

        self._the_board[cell_id - 1] = new_value

    def rows(self) -> list[list[str]]:
        """
        Get the contents of each row... in order.
        """

        return [
            self._the_board[0:3],
            self._the_board[3:6],
            self._the_board[6:9],
        ]

    def columns(self) -> list[list[str]]:
        """
        Get the contents of each column... in order.
        """

        return [
            self._the_board[0::3],
            self._the_board[1::3],
            self._the_board[2::3],
        ]

    def diagonals(self) -> list[list[str]]:
        """
        Get the contents of each diagonal... in order.
        """

        return [
            self._the_board[0:9:4],
            self._the_board[2:7:2],
        ]

    def is_full(self) -> bool:
        """
        Return true if all 9 cells hold player symbols.

        Returns:
            True if every cell in the board has either an 'X' or an 'O'

        """

        return not any(cell.isdigit() for cell in self._the_board)

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
            (
                "|".join(self._the_board[0:3]),
                "|".join(self._the_board[3:6]),
                "|".join(self._the_board[6:9]),
            )
        )
