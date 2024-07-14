import itertools
from typing import Optional, Tuple

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

        triples = itertools.chain(
            self._board_ref.rows(),
            self._board_ref.columns(),
            self._board_ref.diagonals(),
        )
        for triple in triples:
            if self._all_three_match(triple):
                # if they match, grab the 'X' or 'O'
                return triple[0]

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

    @staticmethod
    def _all_three_match(triple: list[str]) -> bool:
        """
        Check for three matching symbols in the Pair-Triple.

        Args:
            triple: set of three cells to check

        Returns:
            True if all three pairs contain the same symbol
        """

        first_symbol = triple[0]
        return all(symbol == first_symbol for symbol in triple)
