from dataclasses import dataclass
from typing import Optional, Protocol, Self

from .builder import Builder
from .player import Player
from .strategy import Strategy


@dataclass
class PlayerBuilder:
    name: Optional[str] = None
    strategy: Optional[Strategy] = None

    @classmethod
    def builder(cls) -> Self:
        return PlayerBuilder()

    def build(self) -> Player:
        return Player(name="", strategy=None)
