from typing import Any, Protocol


# This does not make sense... It will be used to discuss class methods.
class Buildable(Protocol):
    def builder(self) -> "Builder": ...


class Builder(Protocol):
    def validate(self) -> None:
        """
        Run all validation checks and generate exceptions for any failed checks.
        """
        ...

    def build(self) -> Any:
        """
        T.B.W.
        """
        ...
