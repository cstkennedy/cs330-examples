#! /usr/bin/env python3

import sys

def fibonacci_sequence(sequence_length: int):
    """
    Generate the Fibonacci Sequence to the n-th number.
    1 1 2 3 5 8 13 21 34...

    The user must enter a number no smaller than 3 and
    no greater than 200
    """

    # De-Morgan's Law
    # !(index >= 3 && index <= 200)
    # !(index >= 3) || !(index <= 200)
    # (index < 3 || index > 200)
    if sequence_length < 3 or sequence_length > 200:
        raise ValueError(f"{sequence_length:3d} is not between 3 and 200\n")

    fm2 = 1  # n-2 (previous previous) fibonacci number
    fm1 = 1  # n-1 (previous) fibonacci number

    yield fm2
    yield fm1

    for _ in range(3, (sequence_length + 1)):
        f = fm1 + fm2
        fm2 = fm1
        fm1 = f

        yield f


def get_fibonacci_index():
    """
    Check the command line arguments (i.e., sys.argv) for an index. If no index
    was provided prompt the user.

    Returns:
        Fibonacci index (i.e., how many numbers to generate).
    """

    if len(sys.argv) < 2:
        index = input("Generate how many numbers? ")
        index = int(index)  # loosely typed
        print()

    else:
        index = int(sys.argv[1])

    return index


def main():
    index = get_fibonacci_index()

    # Think of the tuple (idx, f) as std::pair<int, long>
    for idx, f in enumerate(fibonacci_sequence(index), start=1):
        print(f"{idx:>3d}: {f:10d}")


if __name__ == "__main__":
    main()
