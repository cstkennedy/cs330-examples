"""
This is a demo driver for a basic single round game of Tic-Tac-Toe.
"""

import logging

from logging_bodge import set_up_logging
from tictactoe import Game, Player
from tictactoe.player import PredefinedMoves


def main() -> None:
    """
    This is a simple main function that takes no command line arguments.
    """

    game = Game(
        player1=Player.create_human(name="Thomas"),
        player2=Player.create_computer(
            name="Jay",
            strategy=PredefinedMoves(moves=[5, 1, 3, 7, 9, 2, 4, 6, 8]),
        ),
    ).play_match()

    print(game)


if __name__ == "__main__":
    set_up_logging(level=logging.INFO)
    main()

    # @todo add exception handling
