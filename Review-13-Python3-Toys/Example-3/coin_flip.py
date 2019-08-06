#! /usr/bin/env python3

import sys
import random

from datetime import datetime
from concurrent.futures import ProcessPoolExecutor

DEFAULT_NUM_TRIALS = 10000
SUMMARY_FMT_STR = "# Heads: {:>6d} ({:>6.4f}) / # Tails {:>6d} ({:>6.4f})"


def simulate_flips(num_trials):
    """
    Simulate a specified number of coin flips

    Keyword arguments:
        num_trials -- number of trials to attempt
    """

    counts = {"Heads": 0, "Tails": 0}

    #  for trial_num in range(0, num_trials):
    for _ in range(0, num_trials):
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

    print(SUMMARY_FMT_STR.format(counts["Heads"],
                                 float(counts["Heads"]) / total_trials,
                                 counts["Tails"],
                                 float(counts["Tails"]) / total_trials))


def run_parallel(num_workers, num_trials):
    """
    Run coin flip simulations in parallel using Python
    Processes
    """

    jobs_per_thread = num_trials // num_workers
    jobs_for_last = jobs_per_thread + (num_trials % num_workers)

    futures = list()

    with ProcessPoolExecutor(max_workers=num_workers) as executor:
        for _ in [jobs_per_thread] * (num_workers - 1) + [jobs_for_last]:
            futures.append(executor.submit(simulate_flips, jobs_per_thread))

    total_heads = 0
    total_tails = 0
    for idx, future in enumerate(futures):
        result = future.result()
        print("Worker {:>2d} -> ".format(idx), end="")
        print_summary(result)

        total_heads += result["Heads"]
        total_tails += result["Tails"]

    print("-" * 72)

    print("Overall   -> ", end="")
    print_summary({"Heads": total_heads, "Tails": total_tails})
    print("Total Trials -> %d" % (total_heads + total_tails))


def main():
    try:
        num_procs = int(sys.argv[1])

    except (IndexError, ValueError) as err:
        num_procs = 1

    if num_procs > 32:
        num_procs = 32

    try:
        num_trials = int(sys.argv[2])

    except (IndexError, ValueError) as err:
        num_trials = DEFAULT_NUM_TRIALS

    if num_procs == 1:
        results = simulate_flips(num_trials)
        print_summary(results)

    else:
        time_parallel = datetime.now()
        run_parallel(num_procs, num_trials)
        time_parallel = (datetime.now() - time_parallel).total_seconds()

        print()
        #  print("Parallel Time: %f" % time_parallel)
        #  print("Parallel Time: {}".format(time_parallel))
        print(f"Parallel Time: {time_parallel:}")


if __name__ == "__main__":
    main()
