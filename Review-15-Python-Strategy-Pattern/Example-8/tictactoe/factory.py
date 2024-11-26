import logging
from typing import Any, Callable, Generic, Optional, Self, Type, TypeVar

from .board import (
    NullRender,
    RenderBigBoardToScreen,
    RenderBoardToScreen,
    RenderStrategy,
)
from .strategy import KeyboardStrategy, MoveStrategy, PredefinedMoves

import tictactoe.factory


S = TypeVar("S")
CreationFunction = Callable[..., S]


class StrategyFactory(Generic[S]):
    """
    Common logic for all strategy factories.
    """

    __strategy_repo: dict[tuple[Type[Any], str], S] = {}
    """
    This stores all strategies for every Factory class that uses
    StrategyFactory as a base.
    """

    @classmethod
    def add(cls, type_of_strategy: str, a_strategy: CreationFunction) -> None:
        if type_of_strategy in cls.__strategy_repo:
            raise ValueError(
                f'An entry for "{type_of_strategy}" already exists'
            )

        cls.__strategy_repo[(cls, type_of_strategy)] = a_strategy  # type: ignore
        logging.info(f"Added '{type_of_strategy}' entry for '{cls}'")

    @classmethod
    def create(cls, type_of_strategy: str, /, **kwargs: Any) -> S:
        if (cls, type_of_strategy) not in cls.__strategy_repo:
            raise ValueError(f'"{type_of_strategy}" is not a known strategy')

        if not kwargs:
            return cls.__strategy_repo[(cls, type_of_strategy)]()

        return cls.__strategy_repo[(cls, type_of_strategy)](**kwargs)

    @classmethod
    def list_strategies(cls) -> str:
        return "\n".join(f"  - {name}" for name in cls.__strategy_repo)


class MoveStrategyFactory(StrategyFactory[MoveStrategy]):
    pass


MoveStrategyFactory.add("Keyboard", KeyboardStrategy)
MoveStrategyFactory.add("SetMoves", PredefinedMoves)


class RenderStrategyFactory(StrategyFactory[RenderStrategy]):
    pass


RenderStrategyFactory.add("Default", RenderBoardToScreen)
RenderStrategyFactory.add("BigBoard", RenderBigBoardToScreen)
RenderStrategyFactory.add("Null", NullRender)
RenderStrategyFactory.add("None", NullRender)


