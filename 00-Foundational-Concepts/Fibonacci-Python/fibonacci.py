#! /usr/bin/env python3

import sys


def __validate_args(index: int) -> bool:
    """
    Note what happens if we do not check the
    index entered by the user.

    De-Morgan's Law
    !(index >= 3 && index <= 20)
    !(index >= 3) || !(index <= 20)
    (index < 3 || index > 20)
    """

    return index < 3 or index > 20


def main():
    """
    Generate the Fibonacci Sequence to the n-th number.
    1 1 2 3 5 8 13 21 34...
    <p>
    The user must enter a number no smaller than 3 and
    no greater than 20
    """

    # --------------------------------------------------------------------------
    #  Prompt for sequence length and validate
    # --------------------------------------------------------------------------
    index = input("Generate how many numbers? ")
    index = int(index)  # loosely typed
    print()

    if __validate_args(index):
        print(f"{index:3d} is not between 3 and 20\n")
        sys.exit(1)

    # --------------------------------------------------------------------------
    #  Compute and output the Fibonaccci Sequence to the index-th term
    # --------------------------------------------------------------------------
    fm2 = 1  # n-2 (previous previous) fibonacci number
    fm1 = 1  # n-1 (previous) fibonacci number

    print(f"{1:>2d}: {fm1:10d}")
    print(f"{2:>2d}: {fm2:10d}")

    # The first 2 numbers were already output
    for i in range(3, (index + 1)):
        f = fm1 + fm2
        fm2 = fm1
        fm1 = f

        print(f"{i:>2d}: {f:10d}")


if __name__ == "__main__":
    try:
        main()
    except:
        # Never write an except block with pass
        pass
