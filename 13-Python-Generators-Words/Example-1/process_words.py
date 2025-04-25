"""
Demonstrate generator expressions in Python using...

  1. a generator function with filter function calls
  2. a generator with conditions
"""

import sys
from typing import Generator, TextIO


def word_stream(word_buffer: TextIO) -> Generator[str, None, None]:
    """
    Read in each line from a buffer (e.g., file) stripping all leading and
    trailing whitespace. All blank lines are skipped.

    Yields:
        lines one at a time, assuming that each line is one word
    """

    for line in word_buffer:
        word = line.strip()
        if word:
            yield word


def contains_chars(word: str, banned: list[str] | None = None) -> bool:
    """
    Determine if a "word" (more aptly "token") contains a banned symbol.

    If no banned list is provided... default to ['.' and '+']

    Returns:
        True if the "word" contains a banned symbol and false otherwise
    """

    if not banned:
        banned = [".", "+"]

    for char in banned:
        if char in word:
            return True

    return False


def main() -> None:
    word_fname = sys.argv[1]

    with open(word_fname, "r") as word_file:
        #  words = word_stream(word_file)
        #  words = filter(lambda word: len(word) < 5, words)
        #  words = filter(lambda word: not contains_chars(word), words)

        words = (
            word
            for word in word_stream(word_file)
            if len(word) < 5
            if not contains_chars(word)
        )

        for word in words:
            print(f"|{word}|")


if __name__ == "__main__":
    main()
