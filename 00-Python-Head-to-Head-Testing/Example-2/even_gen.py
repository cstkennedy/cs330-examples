import sys


def output_even_integers(lower: int, upper: int) -> None:
    """
    Output all even numbers in a given range.

    # Args

      - lower lower integer bound (a)
      - upper upper integer bound (b)
    """

    for next_even in range(lower, upper + 1, 2):
        print(f"{next_even}")


def main():
    # Check and parse command line args
    if len(sys.argv) < 3:
        print(" Usage: ./even_gen [lower_bound] [upper_bound]")
        sys.exit(1)

    # Assume all args are well formed (i.e., can be parsed as integers).
    lower_bound = int(sys.argv[1])
    upper_bound = int(sys.argv[2])

    # The core even output logic
    print(f"Range [{lower_bound}, {upper_bound}]")
    print()

    output_even_integers(lower_bound, upper_bound)


if __name__ == "__main__":
    main()
