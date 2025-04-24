#! /usr/bin/env python3

from __future__ import annotations

import sys
from typing import Optional

# 9.2 ms …  13.4 ms
# 9.1 ms …  13.2 ms
# 9.2 ms …  12.7 ms
# 9.1 ms …  13.6 ms
# 9.6 ms …  15.9 ms

def word_stream(word_fname: str):
    """
    T.B.W.
    """
    with open(word_fname, "r") as word_file:
        for line in word_file:
            word = line.strip()
            if word:
                yield word

def contains_chars(word: str, banned: Optional[list[str]] = None) -> bool:
    if not banned:
        banned = [".", "+"]

    for char in banned:
        if char in word:
            return True

    return False


def main():

    word_fname = sys.argv[1]

    words = word_stream(word_fname)
    #  words_lt_5 = (word for word in words if len(word) < 5)
    words = filter(lambda word: len(word) < 5, words)
    words = filter(lambda word: not contains_chars(word), words)


    for word in words:
        print(f"|{word}|")


if __name__ == "__main__":
    main()
