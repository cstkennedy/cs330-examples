"""
This is a demo driver for a basic single round game of Tic-Tac-Toe.
"""

# -------------------------------------------------------------------------------
# Set up logging before tictactoe.factory is imported... so that
# we can actually log module initialization
# -------------------------------------------------------------------------------
import logging_bodge  # noqa: F401, W0611
import tictactoe.factory as factories
from tictactoe.builders import GameBuilder


def main() -> None:
    """
    This is a simple main function that takes no command line arguments.
    """
    print(
        "\n".join(
            (
                "Known Strategies:",
                factories.MoveStrategyFactory.list_strategies(),
                "",
                "Render Options:",
                factories.RenderStrategyFactory.list_strategies(),
                "",
            )
        )
    )

    # fmt: off
    game = (
        GameBuilder.builder()
        .add_human_player(name="Tom")
        .add_player(
            name="Jay",
            strategy="SetMoves",
            moves=[5, 1, 3, 7, 9, 2, 4, 6, 8]
        )
        .build()
        .play_match()
    )
    # fmt: on

    print(game)


if __name__ == "__main__":
    main()

    # @todo add exception handling
