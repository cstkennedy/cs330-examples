"""
This is a demo driver for a basic single round game of Tic-Tac-Toe.
"""

import logging

from logging_bodge import set_up_logging
#  from tictactoe import Game, Player
#  from tictactoe.player import PredefinedMoves
from tictactoe import Game, Player


def main() -> None:
    #  game = Game.new_with_hardcoded_players().play_match()
    game = Game(
        player1=Player.create_human(name="Thomas"),
        player2=Player.create_computer(
            name="Jay",
            #  strategy=PredefinedMoves(moves=[5, 1, 3, 7, 9, 2, 4, 6, 8]),
            moves=[5, 1, 3, 7, 9, 2, 4, 6, 8],
        ),
    ).play_match()

    #  print(repr(game))
    print(game)


if __name__ == "__main__":
    set_up_logging(level=logging.INFO)
    main()

    # @todo add exception handling
