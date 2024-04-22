from dataclasses import dataclass
from typing import Never, Optional, Protocol, Self, Callable, Any

from .builder import Builder
from .player import Player
from .strategy import KeyboardStrategy, PredefinedMoves, Strategy


class StrategyFactory:
    __strategy_repo = {
        "Keyboard": KeyboardStrategy,
        "SetMoves": PredefinedMoves
    }

    @classmethod
    def add(cls, type_of_strategy: str, a_strategy) -> None:
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
    def builder() -> "PlayerBuilder":
        return PlayerBuilder()

    def with_name(self, val: str) -> Self:
        self.name = val

        return self

    def human(self) -> Self:
        if not self.name:
            raise ValueError("A human player must have a name")

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
            name=self.name,
            strategy=self.strategy,
            humanity=self.is_human
        )
