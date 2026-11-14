"""
This is a demo driver for a basic single round game of Tic-Tac-Toe.
"""

import logging
from dataclasses import dataclass

from logging_bodge import set_up_logging

#  from tictactoe import Game, Player
#  from tictactoe.player import PredefinedMoves
from tictactoe import Game, Player

"""
pyo3_runtime.PanicException: called `Result::unwrap()` on an `Err` value: PyErr { type: <class 'TypeError'>, value: TypeError("FixedMoves.next_move() missing 1 required positional argument: 'self'"), traceback: None }
"""

"""
pyo3_runtime.PanicException: called `Result::unwrap()` on an `Err` value: PyErr { type: <class 'UnboundLocalError'>, value: UnboundLocalError("cannot access local variable 'current_move' where it is not associated with a value"), traceback: Some("Traceback (most recent call last):\n  File \"/home/thomas/Courses/Reviews/cs395-examples/99-FFI-Strategy-Pattern/Example-8/play_tictactoe.py\", line 25, in next_move\n    if current_move > len(self.moves):\n       ^^^^^^^^^^^^\n") }
"""


@dataclass
class FixedMoves:
    moves: list[int]
    #  next_move: int = 0 # this was the issue...
    next_move_idx: int = 0

    def next_move(self) -> int:
        #  if current_move > len(self.moves):
        if self.next_move_idx > len(self.moves):
            raise ValueError("Out of Moves")

        current_move = self.moves[self.next_move_idx]
        self.next_move_idx += 1

        #  return None

        return current_move


def main() -> None:
    #  game = Game.new_with_hardcoded_players().play_match()
    game = Game(
        player1=Player.create_human(name="Thomas"),
        player2=Player.create_custom_computer(
            name="Jay", strategy=FixedMoves(moves=[5, 1, 3, 7, 9, 2, 4, 6, 8])
        ),
    ).play_match()

    #  print(repr(game))
    print(game)


if __name__ == "__main__":
    set_up_logging(level=logging.INFO)
    main()

    # @todo add exception handling
