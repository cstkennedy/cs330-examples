from __future__ import annotations

from typing import Self

import itertools
from dataclasses import dataclass


@dataclass
class BorderHeading:
    content: str
    symbol: str = "~"
    width: int = 38

    def with_width(self, val: int) -> Self:
        return self

    def __str__(self) -> str:
        return "\n".join(
            (
                self.symbol * self.width,
                self.content.center(self.width),
                self.symbol * self.width,
            )
        )


@dataclass
class MultiLineBorderHeading:
    content: tuple[str, ...]
    symbol: str
    width: int

    def __str__(self) -> str:
        return "\n".join(
            itertools.chain(
                [self.symbol * self.width],
                (line.center(self.width) for line in self.content),
                [self.symbol * self.width],
            )
        )
