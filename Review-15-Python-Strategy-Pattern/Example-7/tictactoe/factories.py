import logging
from typing import Any, Callable, Generic, Self, Type, TypeVar

from .board import (
    NullRender,
    RenderBigBoardToScreen,
    RenderBoardToScreen,
    RenderStrategy,
)
from .player import KeyboardStrategy, MoveStrategy, PredefinedMoves

logger = logging.getLogger("tictactoe.factory")

S = TypeVar("S")
CreationFunction = Callable[..., S]


class StrategyFactory(Generic[S]):
    """
    Common logic for all strategy factories.
    """

    __strategy_repo: dict[tuple[Type[Self], str], CreationFunction] = {}
    """
    This stores all strategies for every Factory class that uses
    StrategyFactory as a base.

    Type[Self] is based on RTM-ing for typing of classes
    <https://docs.python.org/3/library/typing.html#the-type-of-class-objects>
    """

    @classmethod
    def add(cls, type_of_strategy: str, a_strategy: CreationFunction) -> None:
        if (cls, type_of_strategy) in cls.__strategy_repo:
            raise ValueError(
                f'An entry for "{type_of_strategy}" already exists'
            )

        cls.__strategy_repo[(cls, type_of_strategy)] = a_strategy  # type: ignore
        logger.info(f"Added '{type_of_strategy}' entry for '{cls}'")

    @classmethod
    def create(cls, type_of_strategy: str, /, **kwargs: Any) -> S:
        if (cls, type_of_strategy) not in cls.__strategy_repo:
            raise ValueError(f'"{type_of_strategy}" is not a known strategy')

        if not kwargs:
            return cls.__strategy_repo[(cls, type_of_strategy)]()

        return cls.__strategy_repo[(cls, type_of_strategy)](**kwargs)

    @classmethod
    def list_strategies(cls) -> str:
        return "\n".join(
            f"  - ({clazz.__name__}) {name}"
            for clazz, name in cls.__strategy_repo
            if clazz.__name__ == cls.__name__
        )

    @classmethod
    def count_strategies(cls) -> int:
        return len(
            list(
                _
                for clazz, _ in cls.__strategy_repo
                if clazz.__name__ == cls.__name__
            )
        )


class MoveStrategyFactory(StrategyFactory[MoveStrategy]):
    @classmethod
    def add_defaults(cls) -> None:
        cls.add("Keyboard", KeyboardStrategy)
        cls.add("SetMoves", PredefinedMoves)


class RenderStrategyFactory(StrategyFactory[RenderStrategy]):
    @classmethod
    def add_defaults(cls) -> None:
        cls.add("Default", RenderBoardToScreen)
        cls.add("BigBoard", RenderBigBoardToScreen)
        cls.add("Null", NullRender)
        cls.add("None", NullRender)
