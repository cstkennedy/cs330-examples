from __future__ import annotations

from itertools import takewhile
import math


def __can_be_divided_by_any(known_primes: list[int], next_prime: int):
    """
    Iterate over all known primes and check the next_prime.

    Returns:
        If next_prime can be evenly divided by any previously known prime
        return True. Return False otherwise
    """

    # While the list of primes has not yet been exhausted
    #  for previous_prime in known_primes:
    for previous_prime in takewhile(lambda p: p <= math.sqrt(next_prime), known_primes):

        # Check for divisibility by the next element--i.e.,
        # if nextPrime % p == 0 for any p, discard nextPrime
        # Is the number prime?
        if next_prime % previous_prime == 0:
            return True

    return False


def generate_primes(to_generate):
    """
    Generate a sequence of prime numbers

    Keyword arguments:
        to_generate -- number of primes to generate
    """

    known_primes = [2, 3]

    for next_prime in known_primes:
        yield next_prime

    for _i in range(3, to_generate + 1):
        # prime from which to start calculations
        next_prime = known_primes[-1]

        # true once a prime number has been identified
        is_prime = False

        # Halt when a prime number has been identified
        while not is_prime:
            # Guess the next prime
            next_prime += 2
            is_prime = not __can_be_divided_by_any(known_primes, next_prime)

        known_primes.append(next_prime)

        yield next_prime
