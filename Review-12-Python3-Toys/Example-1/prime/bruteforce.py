def generate_primes(to_generate):
    """
    Generate a sequence of prime numbers

    Keyword arguments:
        to_generate -- number of primes to generate
    """

    known_primes = [2, 3]

    for next_prime in known_primes:
        yield next_prime

    for i in range(3, to_generate + 1):
        # prime from which to start calculations
        next_prime = known_primes[-1]

        # true once a prime number has been identified
        prime = False

        # Iterate of all existing known prime numbers
        # halt when a prime number has been identified
        while not prime:
            # Guess the next prime
            # Assume the number is not prime
            next_prime += 2
            prime = True

            # While the list of primes has not yet been exhausted
            for previous_prime in known_primes:

                # Check for divisibility by the next element--i.e.,
                # if nextPrime %p == 0 for any p, discard nextPrime

                # Is the number prime?
                prime = (next_prime % previous_prime != 0)

                if not prime:
                    break

        known_primes.append(next_prime)

        yield next_prime
