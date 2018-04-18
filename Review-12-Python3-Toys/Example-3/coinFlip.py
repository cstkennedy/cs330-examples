#! /usr/bin/env python3

import sys
import random


DEFAULT_NUM_TRIALS = 10000


def simulate_flips(num_trials):
    """
    Simulate a specified number of coin flips

    Keyword arguments:
        num_trials -- number of trials to attempt
    """

    counts = {"Heads": 0, "Tails": 0}

    for trial_num in range(0, num_trials):
        result = random.choice(["Heads", "Tails"])

        counts[result] += 1

    return counts


def print_summary(counts):
    """
    Print coin flip statistics (including probability distribution)

    Keyword arguments:
        counts -- heads and tails counts in the form
                  {"Heads": int, "Tails": int}
    """

    total_trials = sum(counts.values())

    print("# Heads: {:>6d} ({:>6.4f}) / # Tails {:>6d} ({:>6.4f})".format(
          counts["Heads"], (float(counts["Heads"]) / total_trials),
          counts["Tails"], (float(counts["Tails"]) / total_trials)))


def main():
    try:
        num_trials = sys.argv[1]
        num_trials = int(num_trials)

    except IndexError:
        num_trials = DEFAULT_NUM_TRIALS

    except ValueError:
        num_trials = DEFAULT_NUM_TRIALS

    results = simulate_flips(num_trials)

    print_summary(results)


if __name__ == "__main__":
    main()
