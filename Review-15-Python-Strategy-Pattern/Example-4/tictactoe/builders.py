from dataclasses import dataclass, field
from typing import Never, Optional, Self

from .game import Game
from .player import Player
from .strategy import KeyboardStrategy, PredefinedMoves, Strategy


class StrategyFactory:
    __strategy_repo = {"Keyboard": KeyboardStrategy, "SetMoves": PredefinedMoves}

    @classmethod
    def add(cls, type_of_strategy: str, a_strategy: Strategy) -> Never:
        if type_of_strategy in cls.__strategy_repo:
            raise ValueError(f'An entry for "{type_of_strategy}" already exists')

        cls.__strategy_repo[type_of_strategy] = a_strategy

    @classmethod
    def create(cls, type_of_strategy: str, /, **kwargs) -> Strategy:
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
    def builder() -> Self:
        return PlayerBuilder()

    def with_name(self, val: str) -> Self:
        self.name = val

        return self

    def human(self) -> Self:
        self.strategy = KeyboardStrategy(self.name)
        self.is_human = True

        return self

    def with_strategy(self, name: str, *args, **kwargs) -> Self:
        self.strategy = StrategyFactory.create(name, **kwargs)

        return self

    def validate(self) -> bool:
        if not self.name:
            return ValueError("No name was set")

        if not self.strategy:
            return ValueError("No strategy was specified")

        return True

    def build(self) -> Player:
        self.validate()

        return Player(name=self.name, strategy=self.strategy, humanity=self.is_human)


@dataclass
class GameBuilder:
    """Unlike the usual "delay object creation until all data is available"
    approach in the builder pattern (i.e., deferring creation until all values
    are available)... the `GameBuilder` immediately creates a `Game` object so that
    `get_board` can be used immediately.

    This allows the board to be passed to Players that need to examine the
    board to generate a move.
    """

    game: Game = field(default_factory=Game)

    player1: Optional[Player] = None
    player2: Optional[Player] = None

    @staticmethod
    def builder() -> Self:
        return GameBuilder()

    def add_human_player(self, *, name) -> Self:
        if self.player1 is not None and self.player2 is not None:
            raise TypeError("Player 1 and Player 2 have already been set")

        player = (
            PlayerBuilder
                .builder()
                .with_name(name)
                .human()
                .build()
        )

        if not self.player1:
            self.player1 = player

        else:
            self.player2 = player

        return self

    def add_player(self, *, name, strategy, **strategy_args) -> Self:
        if self.player1 is not None and self.player2 is not None:
            raise TypeError("Player 1 and Player 2 have already been set")

        player = (
            PlayerBuilder
                .builder()
                .with_name(name)
                .with_strategy(name=strategy, **strategy_args)
                .build()
        )

        if not self.player1:
            self.player1 = player

        else:
            self.player2 = player

        return self

    def validate(self) -> bool:
        return True

    def build(self) -> Game:
        self.validate()

        self.game.set_players(self.player1, self.player2)

        return self.game