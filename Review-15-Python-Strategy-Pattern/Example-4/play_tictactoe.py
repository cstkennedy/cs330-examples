"""
This is a demo driver for a basic single round game of Tic-Tac-Toe.
"""

from typing import Never

from tictactoe import Game, KeyboardStrategy, Player
from tictactoe.builders import PlayerBuilder, StrategyFactory
from tictactoe.strategy import PredefinedMoves


def main():
    """
    This is a simple main function that takes no command line arguments.
    """

    #  tom = Player(name="Thomas", strategy=KeyboardStrategy("Tom"))
    #  jay = Player(name="Jay", strategy=KeyboardStrategy("Jay"))

    tom = (
        PlayerBuilder
            .builder()
            .with_name("Thomas")
            .human()
            .build()
    )

    #  jay = PlayerBuilder.builder().with_name("Jay").human().build()
    """
    jay = (
        PlayerBuilder
            .builder()
            .with_name("Jay")
            .with_strategy(
                PredefinedMoves(
                    moves=[5, 1, 3, 7, 9, 2, 4, 6, 8]
                )
            )
            .build()
    )
    """

    jay = (
        PlayerBuilder
            .builder()
            .with_name("Jay")
            .with_strategy(
                name="SetMoves",
                moves=[5, 1, 3, 7, 9, 2, 4, 6, 8]
            )
            .build()
    )

    """
    tom = (
        PlayerBuilder
            .builder()
            .with_name("Tom")
            .with_strategy(
                name="SetMoves",
                moves=[5, 1, 3, 7, 9, 2, 4, 6, 8]
            )
            .build()
    )
    """



    game = Game()
    game.set_players(tom, jay)
    game.play_match()


if __name__ == "__main__":
    main()

    # @todo add exception handling
