from __future__ import annotations

from typing import Generic, TypeVar

K = TypeVar("K")
V = TypeVar("V")

Entry = (K, V)


class HashMap(Generic[K, V]):
    def __init__(self) -> None:
        raise NotImplementedError()

    def __getitem__(self, key: K) -> V:
        raise NotImplementedError()

    def __setitem__(self, key: K, value: V) -> None:
        raise NotImplementedError()

    def __repr__(self) -> str:
        raise NotImplementedError()
