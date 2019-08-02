#! /usr/bin/env python3

import sys
import random
import copy

from linkedlist import LinkedList

# Tuples are immutable
PROGRAM_HEADING = ("Linked List Review", "Thomas J. Kennedy")

MIN: int = -10  # Lower bound for number generation
MAX: int = +10  # Upper bound for number generation


def main():
    """
    Back to the beginning of the semester... with Linked Lists
    """

    to_generate = 0  # Number of nodes to generate

    # If a seed was passed from the command line,
    # parse it. Otherwise default to ctime
    try:
        random.seed(int(sys.argv[1]))

    except IndexError as err:
        pass

    # If a node count was passed from the command line,
    # parse it. Otherwise default to 10
    try:
        to_generate = int(sys.argv[2])

    except (IndexError, ValueError) as err:
        to_generate = 10

    # Print the program heading
    print("-" * 80)

    for line in PROGRAM_HEADING:
        print(f"{line:^80}")

    print("-" * 80)

    # Create a Linked List
    random_ints = LinkedList()
    for _ in range(0, to_generate):
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
