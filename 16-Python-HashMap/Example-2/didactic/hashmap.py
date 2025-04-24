from __future__ import annotations

from typing import Generic, TypeVar, Iterator

K = TypeVar("K")
V = TypeVar("V")

Entry = (K, V)


class HashMap(Generic[K, V]):
    def __init__(self, num_buckets=256) -> None:
        self.buckets: list[tuple[K, V] | None] = [None] * num_buckets

    def __compute_idx(self, key: K) -> int:
        return hash(key) % len(self.buckets)

    def __getitem__(self, key: K) -> V:
        idx = self.__compute_idx(key)

        if self.buckets[idx] is None:
            raise KeyError(f"No entry for '{key}'")

        return self.buckets[idx][1]

    def __setitem__(self, key: K, value: V) -> None:
        idx = self.__compute_idx(key)

        self.buckets[idx] = (key, value)

    def items(self) -> Iterator[tuple[K, V]]:
        for entry in self.buckets:
            if not entry:
                continue

            yield entry

    def keys(self) -> Iterator[K]:
        for key, _ in self.items():
            yield key

    def values(self) -> Iterator[V]:
        for _, value in self.items():
            yield value

    def __iter__(self) -> Iterator[K]:
        yield from self.keys()

    def __repr__(self) -> str:
        return "{" + ", ".join((f"{key!r}: {value!r}" for key, value in self.items())) + "}"
