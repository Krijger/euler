import itertools


def naturals():
    """
    Generator for natural numbers.

    I chose for itertools.count instead of writing that function myself, which would be something like:

    def naturals():
        x = 0
        while True:
            yield x
            x += 1
    """
    return itertools.count(1, 1)


def triangle_numbers():
    """
    Generator for triangle numbers.

    The sequence of triangle numbers is generated by adding the natural numbers.
    So the 7th triangle number is 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms are:
    1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
    """
    t = 0
    for n in naturals():
        t += n
        yield t


"""
1.  Factorize into primes
2.  Count how many times the primes (not counting 1 to the set of primes) occur, e.g. 28 = 2^2 * 7^1.
    Now, every divisor is any combination of none to all of those primes. That gives a possible number of combinations
    equal to the product of the times the different primes occur plus 1.
    In the case of 28, this is (2 + 1) * (1 + 1) = 6.
    In the case of 21 (3 * 7): (1 + 1) * (1 + 1) = 4.
    In the case of 3 (3): (1 + 1) = 2.
"""

factors = {}


def record_factors(n):
    if n < 2:
        return
    if n in factors:
        return
    for i in itertools.count(2, 1):
        divided = n / i
        if divided.is_integer():
            d = int(divided)
            # note that if n is prime, d will be 1
            if d == 1:
                factors[n] = [i]
            else:
                factors[n] = [d, i]
                record_factors(i)
                record_factors(d)
            return


def prime_factors(n):
    if n < 2:
        return
    if n not in factors:
        record_factors(n)
    fs = factors[n]
    if len(fs) == 1:
        return fs
    else:
        smallers = prime_factors(fs[0]) + prime_factors(fs[1])
        smallers.sort()
        return smallers
