#! /usr/bin/env python3

from __future__ import annotations

import sys
import time
from concurrent.futures import ProcessPoolExecutor
from dataclasses import dataclass

import numpy as np

# Number of nanoseconds in a second
NS_IN_SEC = 10 ** 9

DEFAULT_NUM_TRIALS = 10_000
SUMMARY_FMT_STR = "# Heads: {:>6,d} ({:>6.4f}) / # Tails {:>6,d} ({:>6.4f})"


@dataclass
class FlipResult:
    num_heads: int = 0
    num_tails: int = 0
    total_flips: int = 0

    @property
    def percent_heads(self) -> float:
        return float(self.num_heads) / self.total_flips

    @property
    def percent_tails(self) -> float:
        return float(self.num_tails) / self.total_flips

    def __str__(self):
        return SUMMARY_FMT_STR.format(
            self.num_heads,
            self.percent_heads,
            self.num_tails,
            self.percent_tails
        )


def simulate_flips(num_trials: int) -> FlipResult:
    """
    Simulate a specified number of coin flips

    Args:
        num_trials: number of trials to attempt
    """

    rng = np.random.default_rng()
    flips = rng.integers(2, size=num_trials, dtype=np.int8)

    num_heads = np.count_nonzero(flips)

    counts = FlipResult(
        num_heads = num_heads,
        num_tails = num_trials - num_heads,
        total_flips = num_trials
    )

    return counts


def run_parallel(num_workers: int, num_trials: int):
    """
    Run coin flip simulations in parallel using Python
    Processes
    """

    flips_per_thread = num_trials // num_workers
    flips_for_last = flips_per_thread + (num_trials % num_workers)

    futures = []

    with ProcessPoolExecutor(max_workers=num_workers) as executor:
        for worker_flips in [flips_per_thread] * (num_workers - 1) + [flips_for_last]:
            futures.append(executor.submit(simulate_flips, worker_flips))

    total_heads = 0
    total_tails = 0
    for idx, future in enumerate(futures):
        result = future.result()
        #  print(f"Worker {idx:>2d} -> ", end="")
        #  print(result)

        print(f"Worker {idx:>2d} -> {result}")

        total_heads += result.num_heads
        total_tails += result.num_tails

    print("-" * 72)

    merged_results = FlipResult(
        total_heads,
        total_tails,
        total_heads + total_tails
    )

    print(f"Overall   -> {merged_results}")
    print(f"Total Trials -> {merged_results.total_flips:,}")


def __handle_command_line_args() -> tuple[int, int]:
    try:
        num_procs = int(sys.argv[1])

    except (IndexError, ValueError) as _err:
        num_procs = 1

    # More than 32 threads is excessive
    if num_procs > 32:
        num_procs = 32

    try:
        num_trials = int(sys.argv[2])

    except (IndexError, ValueError) as _err:
        num_trials = DEFAULT_NUM_TRIALS

    return num_procs, num_trials


def main():
    num_procs, num_trials = __handle_command_line_args()

    if num_procs == 1:
        time_sequential = time.perf_counter_ns()

        results = simulate_flips(num_trials)
        print(results)

        time_sequential = (time.perf_counter_ns() - time_sequential) / NS_IN_SEC
        print(f"Sequential Time: {time_sequential:}")

    else:
        time_parallel = time.perf_counter_ns()

        run_parallel(num_procs, num_trials)

        time_parallel = (time.perf_counter_ns() - time_parallel) / NS_IN_SEC
        print(f"Parallel Time: {time_parallel:}")


if __name__ == "__main__":
    main()
