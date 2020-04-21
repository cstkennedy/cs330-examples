#! /usr/bin/env python3


def fibonacci_sequence(sequence_length):
    """
    Generate the Fibonacci Sequence to the n-th number.
    1 1 2 3 5 8 13 21 34...

    The user must enter a number no smaller than 3 and
    no greater than 20
    """

    fm2 = 1  # n-2 (previous previous) fibonacci number
    fm1 = 1  # n-1 (previous) fibonacci number
    f = 0  # current fibonacci number

    yield fm2
    yield fm1

    for _ in range(3, (sequence_length + 1)):
        f = fm1 + fm2
        fm2 = fm1
        fm1 = f

        yield f


def main():
    index = input("Generate how many numbers? ")
    index = int(index)  # loosely typed

    print()

    # De-Morgan's Law
    # !(index >= 3 && index <= 200)
    # !(index >= 3) || !(index <= 200)
    # (index < 3 || index > 200)
    if index < 3 or index > 200:
        print(f"{index:3d} is not between 3 and 200\n")
        exit(1)

    # Think of the tuple (idx, f) as std::pair<int, long>

    for idx, f in enumerate(fibonacci_sequence(index), start=1):
        print(f"{idx:>3d}: {f:10d}")


if __name__ == "__main__":
    main()
