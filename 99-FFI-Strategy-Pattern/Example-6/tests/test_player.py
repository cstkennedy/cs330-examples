from dataclasses import dataclass

import pytest
from hamcrest import assert_that, contains_exactly, equal_to, has_length, is_
from tictactoe import Player


def test_create_human() -> None:
    player = Player.create_human(name="Thomas")

    assert_that(player.name, is_("Thomas"))


def test_create_computer() -> None:
    player = Player.create_computer(
        name="Jay",
        moves=[5, 1, 3, 7, 9, 2, 4, 6, 8],
    )

    assert_that(player.name, is_("Jay"))


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


def test_create_custom_computer_success() -> None:
    player = Player.create_custom_computer(
        name="Jay", strategy=FixedMoves(moves=[5, 1, 3, 7, 9, 2, 4, 6, 8])
    )

    assert_that(player.name, is_("Jay"))


@dataclass
class BrokenStrategy1:
    pass


@dataclass
class BrokenStrategy2:
    next_move: int = 5


def test_create_custom_computer_failure_1() -> None:
    with pytest.raises(AttributeError):
        _ = Player.create_custom_computer(
            name="Jay", strategy=BrokenStrategy1()
        )


def test_create_custom_computer_failure_2() -> None:
    with pytest.raises(TypeError):
        _ = Player.create_custom_computer(
            name="Jay", strategy=BrokenStrategy2()
        )
