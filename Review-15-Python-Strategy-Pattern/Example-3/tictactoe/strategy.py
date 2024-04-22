from dataclasses import dataclass
from typing import Never, Protocol


class Strategy(Protocol):
    def next_move(self) -> int:
        """
        Retrieve the next move.
        """


PROMPT_MSG: str = "Enter your desired move (1-9): "


@dataclass
class KeyboardStrategy:
    _name: str = ""

    def next_move(self) -> int:
        choice = int(input(f"{self._name}, {PROMPT_MSG}"))

        return choice


class PredefinedMoves:
    def __init__(self, *, moves: list[int]) -> None:
        self.my_moves = moves
        self.__move_idx = 0

    def next_move(self) -> int:
        my_next_move = self.my_moves[self.__move_idx]

        self.__move_idx += 1

        return my_next_move
