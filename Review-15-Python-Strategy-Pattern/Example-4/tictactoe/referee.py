from typing import Tuple, Optional

CellPair = Tuple[int, str]
CellTriple = Tuple[CellPair, CellPair, CellPair]


class Referee:
    """
    The Meta-player that checks game status
    e.g., checks for wins, who won, if there is
    a stalemate.

    It is an implementation detail that is not
    exposed to the outside world.
    """

    def __init__(self, board):
        """
        Create the referee and allow access
        to a board through a constant reference variable.

        Args:
            board: the board to monitor
        """

        self._board_ref = board

    def check_for_win(self) -> Optional[str]:
        """
        Check for a win condition.

        Returns:
            "X" if player1, or "O" if player2 won. None indicates neither
            player won.
        """

        winner = self._check_for_horizontal_win()

        if winner:
            return winner

        winner = self._check_for_vertical_win()

        if winner:
            return winner

        winner = self._check_for_diagonal_win()

        if winner:
            return winner

        return None

    def selected_cell_is_empty(self, move: int) -> bool:
        """
        Determine whether a cell in the board has been selected
        by a player.

        Args:
            move: player candidate move

        Returns:
            True if the cell is currently empty
        """

        return self._board_ref.get_cell(move) not in ["X", "O"]

    def _check_for_horizontal_win(self) -> Optional[str]:
        """
        Check each row of the board for three 'X' or three 'O'
        characters.

        Returns:
            "X" if player 1 has won, "O" if player 2 has one, or None if
            no one has won
        """
        for idx_triple in ((1, 2, 3), (4, 5, 6), (7, 8, 9)):
            triple = self._board_ref.get_3_cells(*idx_triple)

            if self._all_three_match(triple):
                # if they match, grab the 'X' or 'O'
                return triple[0][1]

        return None

    def _check_for_vertical_win(self) -> Optional[str]:
        """
        Check each column of the board for three 'X' or three 'O'
        characters.

        Returns:
            1 if player 1 has won, 2 if player 2 has one, or 0 if
            no one has won
        """

        for idx_triple in ((1, 4, 7), (2, 5, 8), (3, 6, 9)):
            triple = self._board_ref.get_3_cells(*idx_triple)

            if self._all_three_match(triple):
                # if they match, grab the 'X' or 'O'
                return triple[0][1]

        return None

    def _check_for_diagonal_win(self) -> Optional[str]:
        """
        Check the two diagonals of the board for three 'X' or three 'O'
        characters.

        Returns:
            "X" if player 1 has won, "O" if player 2 has one, or None if
            no one has won

        """

        for idx_triple in ((1, 5, 9), (7, 5, 3)):
            triple = self._board_ref.get_3_cells(*idx_triple)

            if self._all_three_match(triple):
                # if they match, grab the 'X' or 'O'
                return triple[0][1]

        return None

    @staticmethod
    def _all_three_match(triple: CellTriple) -> bool:
        """
        Check for three matching symbols in the Pair-Triple.

        Args:
            triple: set of three cells to check

        Returns:
            True if all three pairs contain the same symbol
        """

        first_symbol = triple[0][1]  # std::pair first and second
        return all(symbol == first_symbol for _, symbol in triple)


