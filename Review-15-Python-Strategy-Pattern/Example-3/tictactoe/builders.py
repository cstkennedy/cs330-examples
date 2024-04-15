from dataclasses import dataclass
from typing import Never, Optional, Protocol, Self

from .builder import Builder
from .player import Player
from .strategy import KeyboardStrategy, Strategy


class StrategyFactory:
    __strategy_repo = {"Keyboard": KeyboardStrategy()}

    @classmethod
    def add_strategy(cls, type_of_strategy: str, a_strategy: Strategy) -> Never:
        if type_of_strategy in cls.__strategy_repo:
            raise ValueError(f'An entry for "{type_of_strategy}" already exists')

        cls.__strategy_repo[type_of_strategy] = a_strategy

    @classmethod
    def get_strategy(cls, type_of_strategy: str) -> Strategy:
        if type_of_strategy not in cls.__strategy_repo:
            raise ValueError(f'"{type_of_strategy}" is not a known strategy')

        return cls.__strategy_repo[type_of_strategy]

    @classmethod
    def list_strategies(cls) -> str:
        return "\n".join(f"  - {name}" for name in cls.__strategy_repo)


@dataclass
class PlayerBuilder:
    name: Optional[str] = None
    strategy: Optional[Strategy] = None

    @classmethod
    def builder(cls) -> Self:
        return PlayerBuilder()

    def build(self) -> Player:
        return Player(name="", strategy=None)
