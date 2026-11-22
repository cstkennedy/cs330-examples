from dataclasses import dataclass

from tictactoe import Player


def test_create_human() -> None:
    player = Player.create_human(name="Thomas")


def test_create_computer() -> None:
    player = Player.create_computer(
        name="Jay",
        moves=[5, 1, 3, 7, 9, 2, 4, 6, 8],
    )


@dataclass
class FixedMoves:
    moves: list[int]
    next_move_idx: int = 0

    def next_move(self) -> int:
        if self.next_move_idx > len(self.moves):
            raise ValueError("Out of Moves")

        current_move = self.moves[self.next_move_idx]
        self.next_move_idx += 1

        return current_move


def test_create_custom_computer() -> None:
    player = Player.create_custom_computer(
        name="Jay", strategy=FixedMoves(moves=[5, 1, 3, 7, 9, 2, 4, 6, 8])
    )
