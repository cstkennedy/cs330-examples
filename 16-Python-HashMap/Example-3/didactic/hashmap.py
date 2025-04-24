from __future__ import annotations

from typing import Generic, TypeVar, Iterator, Never

K = TypeVar("K")
V = TypeVar("V")

Entry = (K, V)

class OutOfBucketsError(Exception):
    pass


class HashMap(Generic[K, V]):
    def __init__(self, num_buckets=256) -> None:
        self.buckets: list[tuple[K, V] | None] = [None] * num_buckets

    def __compute_idx(self, key: K) -> int:
        return hash(key) % len(self.buckets)

    def __getitem__(self, key: K) -> V:
        idx = self.__compute_idx(key)

        if self.buckets[idx] is None:
            raise KeyError(f"No entry for '{key}'")

        if key == self.buckets[idx][0]:
            return self.buckets[idx][1]

        # Handle collision
        for offset, bucket in enumerate(self.buckets[idx+1:], start=idx+1):
            if not bucket:
                raise KeyError(f"No entry for '{key}'")

            if  key == bucket[0]:
                return bucket[1]

        raise KeyError(f"No entry for '{key}'")

    def __setitem__(self, key: K, value: V) -> None:
        idx = self.__compute_idx(key)

        # No entry exists for "key"
        if not self.buckets[idx]:
            self.buckets[idx] = (key, value)
            return

        # Key is already present (update value)
        if key == self.buckets[idx][0]:
            self.buckets[idx][1] = value
            return

        # Collision - bucket[idx]'s current key does not match the key argument
        # find first empty bucket or a bucket with matching key
        for offset, bucket in enumerate(self.buckets[idx+1:], start=idx+1):
            # If an empty bucket is encountered... store the new entry there
            if not bucket:
                self.buckets[offset] = (key, value)
                return

            # Find a non-empty bucket
            if bucket and key == bucket[0]:
                bucket[1] = value
                return

        raise OutOfBucketsError()

    def __delitem__(self, key: K) -> Never:
        raise NotImplementedError()

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
