class NotSetValueError(ValueError):
    """
    Used when a ValueError is results from when None is encountered for a
    required value
    """

class BuildValueError(ValueError):
    """Used when a ValueError is generated in a builder's build method"""


