#! /usr/bin/env python3

"""
This is a demo driver for a basic single round game of Tic-Tac-Toe.
"""

from examples import Game, Player


def main():
    """
    This is a simple main function that takes no command line arguments.
    """

    tom = Player("Thomas")
    jay = Player("Jay")

    game = Game(tom, jay)

    while game.is_not_over():
        game.play_round()

    print(game.get_board())

    if game.ended_with_win():
        print("Congratulations {}!".format(game.get_winner()))


if __name__ == "__main__":
    main()

    # @todo add exception handling
