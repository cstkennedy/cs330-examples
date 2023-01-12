#! /usr/bin/env python3

from __future__ import annotations

import sys


def stream_large_file(word_filename: str):
    with open(word_filename, "r") as word_file:
        for line in word_file:
            word = line.strip()
            if len(word) == 0:
                continue

            yield word

def contains_chars(word: str, disallowed: list[str]) -> bool:
    for sym in disallowed:
        if sym in word:
            return True
    return False


def main():
    try:
        word_filename = sys.argv[1]
    except IndexError as _err:
        print(f"Usage: {sys.argv[0]} filename")
        sys.exit(1)

    #  for word in stream_large_file(word_filename):
        #  if len(word) <= 4: 
            #  if "." not in word and "+" not in word:
                #  print(word)

    word_list = (word for word in stream_large_file(word_filename))
    word_list = filter(lambda word: len(word) <= 4, word_list)
    word_list = filter(lambda word: not contains_chars(word, [".", "+"]), word_list)

    for word in word_list:
        print(word)

if __name__ == "__main__":
    main()
