#! /usr/bin/env python3

import sys
import random
import copy

from linkedlist import LinkedList

PROGRAM_HEADING = "\n".join(
    ("Linked List Review".center(80), "Thomas J. Kennedy".center(80))
)

MIN: int = -10  # Lower bound for number generation
MAX: int = +10  # Upper bound for number generation


def get_number_to_generate() -> int:
    """
    Get the the number of values to generate.

    Returns:
        integer supplied by the user iff one was supplied at the command line.
        Otherwise return the default value.
    """

    # If a node count was passed from the command line,
    # parse it. Otherwise default to 10
    try:
        to_generate = int(sys.argv[2])

    except (IndexError, ValueError) as _err:
        to_generate = 10

    return to_generate


def set_up_random() -> None:
    """
    If a seed was passed from the command line,
    parse it. Otherwise default to ctime
    """

    try:
        random.seed(int(sys.argv[1]))

    except IndexError as _err:
        pass


def main():
    """
    Back to the beginning of the semester... with Linked Lists
    """

    print("-" * 80)
    print(PROGRAM_HEADING)
    print("-" * 80)

    set_up_random()
    initial_data = [random.randint(MIN, MAX) for _ in range(get_number_to_generate())]
    random_ints = LinkedList(*initial_data)

    print(random_ints)

    random_copy = copy.deepcopy(random_ints)
    random_copy.append(337)

    print("*" * 80)
    print(random_copy)

    print("*" * 80)
    print(random_ints)

    print("#" * 80)
    print("Iterators!".center(80))
    print("#" * 80)

    for idx, value in enumerate(random_ints):
        print(f"Node # {idx:>4} - {value:>4}")


if __name__ == "__main__":
    main()
