#! /usr/bin/env python3

from __future__ import annotations

import random
import sys

import coin_flip_py as coin_flip

DEFAULT_NUM_TRIALS = 10000
SUMMARY_FMT_STR = "# Heads: {:>6d} ({:>6.4f}) / # Tails {:>6d} ({:>6.4f})"


def main():
    try:
        num_threads = int(sys.argv[1])

    except (IndexError, ValueError) as _err:
        num_threads = 1

    if num_threads > 32:
        num_threads = 32

    try:
        num_trials = int(sys.argv[2])

    except (IndexError, ValueError) as _err:
        num_trials = DEFAULT_NUM_TRIALS

    coin_flip.do_flips(num_threads, num_trials)

if __name__ == "__main__":
    main()
