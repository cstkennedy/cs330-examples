import sys


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

    for next_even in range(lower_bound, upper_bound + 1):
        print(f"{next_even}")


if __name__ == "__main__":
    main()
