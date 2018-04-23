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
    f   = 0  # current fibonacci number

    yield fm2
    yield fm1

    for i in range(3, (sequence_length + 1)):
        f   = fm1 + fm2
        fm2 = fm1
        fm1 = f

        yield f


def main():
    # Prompt the user
    index = input("Generate how many numbers? ")
    index = int(index)  # loosely typed

    # Print a blank line
    print()

    # De-Morgan's Law
    # !(index >= 3 && index <= 200)
    # !(index >= 3) || !(index <= 200)
    # (index < 3 || index > 200)
    if index < 3 or index > 200:
        print("{:3d} is not between 3 and 20\n".format(index))
        exit(1)

    for idx, f in enumerate(fibonacci_sequence(index)):
        print("{:>2d}: {:10d}".format(idx, f))


if __name__ == "__main__":
    try:
        main()
    except:
        pass
