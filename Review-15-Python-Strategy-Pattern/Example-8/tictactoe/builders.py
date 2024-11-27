from dataclasses import dataclass
from typing import Optional, Self

from .factory import MoveStrategyFactory, RenderStrategyFactory
from .game import Game
from .player import Player
from .strategy import MoveStrategy


@dataclass
class PlayerBuilder:
    name: Optional[str] = None
    strategy: Optional[MoveStrategy] = None
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

        self.strategy = MoveStrategyFactory.create("Keyboard", _name=self.name)
        self.is_human = True

        return self

    def with_strategy(self, name: str, *_args, **kwargs) -> Self:
        self.strategy = MoveStrategyFactory.create(name, **kwargs)

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
                RenderStrategyFactory.create("BigBoard")
                if self.is_human
                else RenderStrategyFactory.create("Null")
            ),
        )


@dataclass
class GameBuilder:
    player1: Optional[Player] = None
    player2: Optional[Player] = None

    @staticmethod
    def builder() -> "GameBuilder":
        return GameBuilder()

    def __add_player_impl(self, player: Player) -> None:
        if self.player1 is not None and self.player2 is not None:
            raise TypeError("Player 1 and Player 2 have already been set")

        if not self.player1:
            self.player1 = player

        else:
            self.player2 = player

    def add_human_player(self, *, name: str) -> Self:
        # fmt: off
        self.__add_player_impl(
            PlayerBuilder.builder()
            .with_name(name)
            .human()
            .build()
        )
        # fmt: on

        return self

    def add_player(self, *, name: str, strategy: str, **strategy_args) -> Self:
        self.__add_player_impl(
            PlayerBuilder.builder()
            .with_name(name)
            .with_strategy(name=strategy, **strategy_args)
            .build()
        )

        return self

    def validate(self) -> None:
        if self.player1 is None:
            raise ValueError("Player 1 was not set")

        if self.player2 is None:
            raise ValueError("Player 2 was not set")

    def build(self) -> Game:
        self.validate()

        return Game(player1=self.player1, player2=self.player2)  # type: ignore
