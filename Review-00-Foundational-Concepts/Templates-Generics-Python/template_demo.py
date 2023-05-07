from __future__ import annotations
from typing import Any


def print_multiple_times(value: Any, count: int):
    """
    Print a value multiple times.

    @param <T> type to print multiple times

    @param value value to print
    @param count number of times to print
    """

    for _ in range(0, count - 1):
        print(f"{value} ", end="")

    print(f"{value}")


def main():
    print_multiple_times(7, 3)
    print()
    print_multiple_times(5.5, 2)
    print()
    print_multiple_times("Hello", 4)
    print()
    print_multiple_times("?", 20)


if __name__ == "__main__":
    main()
