#! /usr/bin/env python3

import sys
import random

DEFAULT_NUM_TRIALS = 10000

def simulate_flips(num_trials):
    
    counts = {"Heads": 0, "Tails": 0}

    for trial_num in range(0, num_trials):
        result = random.choice(["Heads", "Tails"])

        counts[result] += 1

    return counts


def print_summary(counts):
    total_trials = sum(counts.values())

    print("# Heads: {:>6d} ({:>6.4f}) / # Tails {:>6d} ({:>6.4f})".format(
         counts["Heads"], (float(counts["Heads"]) / total_trials),
         counts["Tails"], (float(counts["Tails"]) / total_trials)))

def main():

    try:
        num_trials = sys.argv[1];
        num_trials = int(num_trials)
    
    except IndexError:
        num_trials = DEFAULT_NUM_TRIALS

    except ValueError:
        num_trials = DEFAULT_NUM_TRIALS

    results = simulate_flips(num_trials)

    print_summary(results)


if __name__ == "__main__":
    main()

