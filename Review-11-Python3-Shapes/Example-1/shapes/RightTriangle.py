import math
import copy

from shapes.Shape import Shape
from shapes.Triangle import Triangle


class RightTriangle(Triangle):
    """
    A Triangle with a hypotenuse and height as two
    of its sides
    """

    ONE_HALF = 1.0 / 2.0  # @f$ \frac{1}{2} @f$

    @staticmethod
    def __computeHypotenuse(base, height):
        """
        Compute the hyptoenuse using:
        @f$ hypotenuse = \sqrt{base^2 + height^2} @f$
        :base: the base of a Right Triangle
        :param: height the height of a Right Triangle

        :return: hypotenuse of a right triangle
        """
        return math.sqrt(
            (base ** 2) + (height ** 2)
        )

    def __init__(self, base=1, height=1):
        """
        Construct a RightTriangle
        with base and height set to 1 by
        default.
        """
        self.name = "Right Triangle"

        self.side_a = base
        self.side_b = height
        self.side_c = RightTriangle.__computeHypotenuse(
            self.side_a,
            self.side_b
        )

    @property
    def base(self):
        return self.side_a

    @base.setter
    def base(self, side):

        self.side_a = side

        self.side_c = RightTriangle.__computeHypotenuse(
            self.side_a,
            self.side_b
        )

    @property
    def height(self):
        self.side_b

    @height.setter
    def height(self, side):
        self.side_b = side

        self.side_c = RightTriangle.__computeHypotenuse(
            self.side_a,
            self.side_b
        )

    @property
    def hypotenuse(self):
        return self.side_c

    def area(self):
        """
        Compute the area using
        @f$ Area = \frac{1}{2}*base*height @f$

        :return: the area
        """

        return RightTriangle.ONE_HALF * self.side_a * self.side_b

    def __deepcopy__(self, memo):
        """
        Return a new duplicate Shape
        """

        return RightTriangle(
            copy.deepcopy(self.side_a),
            copy.deepcopy(self.side_b),
        )

    def __str__(self):
        formatStr = (
            "{:<" +
            str(Shape.WIDTH_LABEL) +
            "}:{:>" +
            str(Shape.WIDTH_VALUE) +
            ".4f}\n"
        )

        return (
            super(Triangle, self).__str__() +
            formatStr.format("Base", self.side_a) +
            formatStr.format("Height", self.side_b) +
            formatStr.format("Hypotenuse", self.side_c) +
            formatStr.format("Perimeter", self.perimeter()) +
            formatStr.format("Area", self.area())
        )

if __name__ == "__main__":

    s = RightTriangle(1, 2)
    print(s)
