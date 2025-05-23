from dataclasses import dataclass
from typing import Any, ClassVar, Self

from .factories import MoveStrategyFactory, RenderStrategyFactory
from .game import Game
from .player import MoveStrategy, Player


@dataclass
class PlayerBuilder:
    name: str | None = None
    strategy: MoveStrategy | None = None
    is_human: bool = False

    defaults_set_up: ClassVar[bool] = False

    @staticmethod
    def builder() -> "PlayerBuilder":
        return PlayerBuilder()

    @classmethod
    def use_defaults(cls) -> None:
        if cls.defaults_set_up:
            return

        MoveStrategyFactory.add_defaults()
        RenderStrategyFactory.add_defaults()
        cls.defaults_set_up = True

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

    def with_strategy(
        self, name: str, *_args: None, **kwargs: dict[str, Any]
    ) -> Self:
        self.strategy = MoveStrategyFactory.create(name, **kwargs)

        return self

    def build(self) -> Player:
        if not self.name:
            raise ValueError("No name was set")

        if not self.strategy:
            raise ValueError("No strategy was specified")

        return Player(
            name=self.name,
            strategy=self.strategy,
            humanity=self.is_human,
            preferred_renderer=(
                RenderStrategyFactory.create("BigBoard")
                if self.is_human
                else RenderStrategyFactory.create("Null")
            ),
        )


@dataclass
class GameBuilder:
    player1: Player | None = None
    player2: Player | None = None

    @staticmethod
    def builder() -> "GameBuilder":
        return GameBuilder()

    def use_defaults(self) -> Self:
        PlayerBuilder.use_defaults()

        return self

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

    def add_player(
        self, *, name: str, strategy: str, **strategy_args: Any
    ) -> Self:
        self.__add_player_impl(
            PlayerBuilder.builder()
            .with_name(name)
            .with_strategy(name=strategy, **strategy_args)
            .build()
        )

        return self

    def build(self) -> Game:
        if self.player1 is None:
            raise ValueError("Player 1 was not set")

        if self.player2 is None:
            raise ValueError("Player 2 was not set")

        return Game(player1=self.player1, player2=self.player2)  # type: ignore
