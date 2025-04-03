import itertools

from .board import Board


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


class Referee:
    """
    The Meta-player that checks game status
    e.g., checks for wins, who won, if there is
    a stalemate.

    It is an implementation detail that is not
    exposed to the outside world.
    """

    @staticmethod
    def check_for_win(board_ref: Board) -> bool:
        """
        Check for a win condition (i.e., three in a row).

        Returns:
            True if a player has won and False otherwise
        """

        triples = itertools.chain(
            board_ref.rows(),
            board_ref.columns(),
            board_ref.diagonals(),
        )

        # if they match, grab the 'X' or 'O'
        if next(filter(_all_three_match, triples), None):
            return True

        return False

    @staticmethod
    def selected_cell_is_empty(board_ref: Board, move: int) -> bool:
        """
        Determine whether a cell in the board has been selected
        by a player.

        Args:
            move: player candidate move

        Returns:
            True if the cell is currently empty
        """

        return board_ref.get_cell(move).isdigit()
