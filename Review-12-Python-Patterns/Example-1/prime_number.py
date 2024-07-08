#! /usr/bin/env python3

# Programmer : Thomas J. Kennedy

import sys

import prime.bruteforce as bruteforce

PROGRAM_HEADING = ("Prime Number Generation", "Thomas J. Kennedy")


def __parse_args() -> int:
    """
    Parse command line arguments (num_primes). Default to 10.
    """

    try:
        num_primes = int(sys.argv[1])

    except (IndexError, ValueError) as _err:
        num_primes = 10

    return num_primes


def main():
    """
    The main function. In practice I could name this
    anything. The name main was selected purely
    out of familiarity.

    The "if __name__" line below determines what runs

    """

    # Print Program Heading
    print("-" * 80)
    for line in PROGRAM_HEADING:
        print(f"{line:^80}")
    print("-" * 80)

    num_primes = __parse_args()

    for prime_num in bruteforce.generate_primes(num_primes):
        print(prime_num)


if __name__ == "__main__":
    main()
