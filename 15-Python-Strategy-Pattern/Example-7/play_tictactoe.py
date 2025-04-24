"""
This is a demo driver for a basic single round game of Tic-Tac-Toe.
"""

import logging

import tictactoe.factories as factories
from logging_bodge import set_up_logging
from tictactoe import Game, Player


def main() -> None:
    """
    This is a simple main function that takes no command line arguments.
    """

    Game.use_defaults()

    print(
        "\n".join(
            (
                f"Known Strategies ({factories.MoveStrategyFactory.count_strategies()}): ",
                factories.MoveStrategyFactory.list_strategies(),
                "",
                f"Render Options ({factories.RenderStrategyFactory.count_strategies()}):",
                factories.RenderStrategyFactory.list_strategies(),
                "",
            )
        )
    )

    game = Game(
        player1=Player.create_human(name="Thomas"),
        player2=Player.create_computer(
            name="Jay",
            strategy=factories.MoveStrategyFactory.create(
                "SetMoves", moves=[5, 1, 3, 7, 9, 2, 4, 6, 8]
            ),
        ),
    ).play_match()

    print(game)


if __name__ == "__main__":
    set_up_logging(level=logging.INFO)
    main()

    # @todo add exception handling
