#! /usr/bin/env python3

# Programmer : Thomas Kennedy

import sys

import prime.bruteforce as bruteforce


PROGRAM_HEADING = [
    "Prime Number Generation",
    "Thomas J. Kennedy"
]  # Program Title


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
        print("{:^80}".format(line))

    print("-" * 80)

    try:
        num_primes = int(sys.argv[1])

    except IndexError:
        num_primes = 10

    except ValueError:
        num_primes = 10

    for prime_num in bruteforce.generate_primes(num_primes):
        print(prime_num)


if __name__ == "__main__":
    main()
