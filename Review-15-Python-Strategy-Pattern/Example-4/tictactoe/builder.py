from typing import Protocol, TypeVar


# This does not make sense... It will be used to discuss class methods.
class Buildable(Protocol):
    def builder(self) -> "Builder": ...


T = TypeVar("T")


class Builder(Protocol[T]):
    def validate(self) -> bool:
        """
        Run all validation checks and generate exceptions for any failed checks.
        """
        ...

    def build(self) -> T:
        """
        T.B.W.
        """
        ...
