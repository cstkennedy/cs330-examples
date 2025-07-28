from typing import Any


class HtmlColor:
    """
    An HTML Color is represented by a combination of red, green and blue. Each
    component (red, green, and blue) must fall in the range [0, 255].

    Any value outside the range [0, 255] is clamped.
    """

    @staticmethod
    def clamp(val: int) -> int:
        """
        Clamp is an implementation of the mathematical operation that takes a
        value and ensures is falls within a specified range.

          - Any value that is less than 0 will be replaced with zero.
          - Any value that is greater than 255 will be replaced with 255.
        """

        return min(max(0, val), 255)

    def __init__(self, red: int = 0, green: int = 0, blue: int = 0) -> None:
        self.red = red
        self.green = green
        self.blue = blue

    @property
    def red(self) -> int:
        return self.__red

    @property
    def green(self) -> int:
        return self.__green

    @property
    def blue(self) -> int:
        return self.__blue

    @red.setter
    def red(self, val: int) -> None:
        self.__red = HtmlColor.clamp(val)

    @green.setter
    def green(self, val: int) -> None:
        self.__green = HtmlColor.clamp(val)

    @blue.setter
    def blue(self, val: int) -> None:
        self.__blue = HtmlColor.clamp(val)

    def __eq__(self, rhs: Any) -> bool:
        """
        Two colors are considered equal if their red, green, and blue
        components are equivalent.
        """

        if not isinstance(rhs, HtmlColor):
            return False

        return (
            self.__red == rhs.red
            and self.__green == rhs.green
            and self.__blue == rhs.blue
        )

    def __hash__(self) -> int:
        return hash((self.__red, self.__green, self.__blue))

    def __str__(self) -> str:
        """
        Colors are represented in the form #RRGGBB where RR, GG, and BB are the
        red, green and blue components, respectively, in base 16 (hexadecimal).
        """

        return f"#{self.__red:02X}{self.__green:02X}{self.__blue:02X}"
