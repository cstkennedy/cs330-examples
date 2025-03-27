class BuildError(ValueError):
    """
    Used as the base for any exception that occurs within a builder
    """


class NotSetError(BuildError):
    """
    Used when a required value is expected, but `None` is encountered
    """


class InvariantError(BuildError):
    """
    Used when a invariant is violated
    """
