from dataclasses import dataclass
from typing import ClassVar, Protocol


class MoveStrategy(Protocol):
    def next_move(self) -> int:
        """
        Retrieve the next move.
        """


@dataclass
class KeyboardStrategy:
    PROMPT_MSG: ClassVar[str] = "Enter your desired move (1-9): "

    _name: str = ""

    def next_move(self) -> int:
        print()
        choice = int(input(f"{self._name}, {self.PROMPT_MSG}"))
        print()

        return choice


class PredefinedMoves:
    def __init__(self, *, moves: list[int]) -> None:
        self.my_moves = moves
        self.__move_idx = 0

    def next_move(self) -> int:
        my_next_move = self.my_moves[self.__move_idx]

        self.__move_idx += 1

        return my_next_move
