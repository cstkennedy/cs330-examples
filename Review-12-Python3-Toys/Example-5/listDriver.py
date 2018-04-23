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


def randomInt(min=MIN, max=MAX):
    """
    Generate a random integer in the range min, max.
    Default to MIN and MAX
    """

    return random.randint(min, max)


def generateList(n):
    """
    Generate a Linked List of random integers

    :param n: number of integers to generate
    """

    ll = LinkedList()

    # Generate a Linked List (LL) of 10 Nodes
    for i in range(0, n):
        ll.appendNode(randomInt())

    return ll


def main():
    """
    Back to the beginning of the semester... with Linked Lists
    """

    toGenerate = 0  # Number of nodes to generate

    # If a seed was passed from the command line,
    # parse it. Otherwise default to ctime
    try:
        random.seed(int(sys.argv[1]))

    except IndexError:
        pass

    # If a node  count was passed from the command line,
    # parse it. Otherwise default to 10
    try:
        toGenerate = int(sys.argv[2])

    except (IndexError, ValueError) as e:
        toGenerate = 10

    # Print the program heading
    print("-" * 80)

    for line in PROGRAM_HEADING:
        print("{:^80}".format(line))

    print("-" * 80)

    # Create a Linked List
    randomInts = generateList(toGenerate)
    print(randomInts)

    print("*" * 80)

    randomCopy = copy.deepcopy(randomInts)

    randomCopy.appendNode(337)

    print(randomCopy)

    print("*" * 80)
    print(randomInts)

    print("#" * 80)
    print("{:^80}".format("Iterators!"))
    print("#" * 80)

    for idx, value in enumerate(randomInts):
        print("Node # {:>4} - {:>4}".format(idx, value))


if __name__ == "__main__":
    main()
