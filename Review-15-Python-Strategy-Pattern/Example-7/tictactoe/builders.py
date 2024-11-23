from dataclasses import dataclass
from typing import Any, Callable, Optional, Self

from .board import NullRender, RenderBoardToScreen
from .game import Game
from .player import Player
from .strategy import KeyboardStrategy, PredefinedMoves, Strategy

StrategyCreationFunction = Callable[..., Strategy]


class StrategyFactory:
    __strategy_repo: dict[str, StrategyCreationFunction] = {
        "Keyboard": KeyboardStrategy,
        "SetMoves": PredefinedMoves,
    }

    @classmethod
    def add(
        cls, type_of_strategy: str, a_strategy: StrategyCreationFunction
    ) -> None:
        if type_of_strategy in cls.__strategy_repo:
            raise ValueError(
                f'An entry for "{type_of_strategy}" already exists'
            )

        cls.__strategy_repo[type_of_strategy] = a_strategy  # type: ignore

    @classmethod
    def create(cls, type_of_strategy: str, /, **kwargs: Any) -> Strategy:
        if type_of_strategy not in cls.__strategy_repo:
            raise ValueError(f'"{type_of_strategy}" is not a known strategy')

        if not kwargs:
            return cls.__strategy_repo[type_of_strategy]()

        return cls.__strategy_repo[type_of_strategy](**kwargs)

    @classmethod
    def list_strategies(cls) -> str:
        return "\n".join(f"  - {name}" for name in cls.__strategy_repo)


@dataclass
class PlayerBuilder:
    name: Optional[str] = None
    strategy: Optional[Strategy] = None
    is_human = False

    @staticmethod
    def builder() -> "PlayerBuilder":
        return PlayerBuilder()

    def with_name(self, val: str) -> Self:
        self.name = val

        return self

    def human(self) -> Self:
        if not self.name:
            raise ValueError(
                "Player name must be set before strategy selection"
            )

        self.strategy = KeyboardStrategy(self.name)
        self.is_human = True

        return self

    def with_strategy(self, name: str, *args, **kwargs) -> Self:
        self.strategy = StrategyFactory.create(name, **kwargs)

        return self

    def validate(self) -> bool:
        if not self.name:
            raise ValueError("No name was set")

        if not self.strategy:
            raise ValueError("No strategy was specified")

        return True

    def build(self) -> Player:
        self.validate()

        return Player(
            name=self.name,  # type: ignore
            strategy=self.strategy,  # type: ignore
            humanity=self.is_human,
            preferred_renderer=(
                RenderBoardToScreen() if self.is_human else NullRender()  # type: ignore
            ),
        )


@dataclass
class GameBuilder:
    """
    Unlike the usual "delay object creation until all data is available"
    approach in the builder pattern (i.e., deferring creation until all values
    are available)... the `GameBuilder` immediately creates a `Game` object so
    that `get_board` can be used immediately.

    This allows the board to be passed to Players that need to examine the
    board to generate a move.
    """

    player1: Optional[Player] = None
    player2: Optional[Player] = None

    @staticmethod
    def builder() -> "GameBuilder":
        return GameBuilder()

    def add_human_player(self, *, name: str) -> Self:
        if self.player1 is not None and self.player2 is not None:
            raise TypeError("Player 1 and Player 2 have already been set")

        # fmt: off
        player = (
            PlayerBuilder.builder()
            .with_name(name)
            .human()
            .build()
        )
        # fmt: on

        if not self.player1:
            self.player1 = player

        else:
            self.player2 = player

        return self

    def add_player(self, *, name: str, strategy: str, **strategy_args) -> Self:
        if self.player1 is not None and self.player2 is not None:
            raise TypeError("Player 1 and Player 2 have already been set")

        player = (
            PlayerBuilder.builder()
            .with_name(name)
            .with_strategy(name=strategy, **strategy_args)
            .build()
        )

        if not self.player1:
            self.player1 = player

        else:
            self.player2 = player

        return self

    def validate(self) -> None:
        if self.player1 is None:
            raise ValueError("Player 1 was not set")

        if self.player2 is None:
            raise ValueError("Player 2 was not set")

    def build(self) -> Game:
        self.validate()

        return Game(self.player1, self.player2)  # type: ignore
