"""
This is a demo driver for a basic single round game of Tic-Tac-Toe.
"""

from examples import Game, KeyboardStrategy, Player


def main():
    """
    This is a simple main function that takes no command line arguments.
    """

    tom = Player(name="Thomas", strategy=KeyboardStrategy("Tom"))
    jay = Player(name="Jay", strategy=KeyboardStrategy("Jay"))

    game = Game()
    game.set_players(tom, jay)
    game.play_match()


if __name__ == "__main__":
    main()

    # @todo add exception handling
