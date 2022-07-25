#! /usr/bin/env python3

import sys
import random
import copy

from linkedlist import LinkedList

PROGRAM_HEADING = [
    "Linked List Review",
    "Thomas J. Kennedy"
]


MIN = -10  # Lower bound for number generation
MAX =  10  # Upper bound for number generation


def random_int(min=MIN, max=MAX):
    """
    Generate a random integer in the range min, max.
    Default to MIN and MAX
    """

    return random.randint(min, max)


def generate_list(n):
    """
    Generate a Linked List of random integers

    :param n: number of integers to generate
    """

    ll = LinkedList()

    # Generate a Linked List (LL) of n Nodes
    for i in range(0, n):
        ll.append(random_int())

    return ll


def main():
    """
    Back to the beginning of the semester... with Linked Lists
    """

    to_generate = 0  # Number of nodes to generate

    # If a seed was passed from the command line,
    # parse it. Otherwise default to ctime
    try:
        random.seed(int(sys.argv[1]))

    except IndexError as _err:
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
    random_ints = generate_list(to_generate)
    print(random_ints)

    print("*" * 80)

    random_copy = copy.deepcopy(random_ints)

    random_copy.append(337)

    print(random_copy)

    print("*" * 80)
    print(random_ints)


if __name__ == "__main__":
    main()
