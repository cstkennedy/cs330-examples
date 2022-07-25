#! /usr/bin/env python3

import sys
import random
import copy

from linkedlist import LinkedList

# Tuples are immutable
PROGRAM_HEADING = ("Linked List Review", "Thomas J. Kennedy")

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

    except (IndexError, ValueError) as err:
        to_generate = 10

    return to_generate


def set_up_random() -> None:
    """
    If a seed was pasVsed from the command line,
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

    set_up_random()

    # Print the program heading
    print("-" * 80)

    for line in PROGRAM_HEADING:
        print(f"{line:^80}")

    print("-" * 80)

    # Create a Linked List
    nodes_to_generate = get_number_to_generate()
    random_ints = LinkedList()
    #  for i in range(0, nodes_to_generate):
    for _ in range(0, nodes_to_generate):
        random_ints.append(random.randint(MIN, MAX))

    print(random_ints)

    print("*" * 80)

    random_copy = copy.deepcopy(random_ints)
    random_copy.append(337)
    print(random_copy)

    print("*" * 80)
    print(random_ints)

    print("#" * 80)
    print("{:^80}".format("Iterators!"))
    print("#" * 80)

    for idx, value in enumerate(random_ints):
        print(f"Node # {idx:>4} - {value:>4}")


if __name__ == "__main__":
    main()
