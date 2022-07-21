import math
import copy

from shapes.Shape import (Shape, FPT_FMT)
from shapes.Triangle import Triangle


class RightTriangle(Triangle):
    """
    A Triangle with a hypotenuse and height as two
    of its sides
    """

    ONE_HALF = 1.0 / 2.0  # @f$ \frac{1}{2} @f$

    @staticmethod
    def __compute_hypotenuse(base, height):
        """
        Compute the hypotenuse using:
        @f$ hypotenuse = \sqrt{base^2 + height^2} @f$
        :param base: the base of a Right Triangle
        :param height: the height of a Right Triangle

        :return: hypotenuse of a right triangle
        """
        return math.sqrt((base ** 2) + (height ** 2))

    def __init__(self, base=1, height=1):
        """
        Construct a RightTriangle
        with base and height set to 1 by
        default.
        """
        self.name = "Right Triangle"

        self.side_a = base
        self.side_b = height
        self.side_c = RightTriangle.__compute_hypotenuse(self.side_a,
                                                         self.side_b)

    @property
    def base(self):
        return self.side_a

    @base.setter
    def base(self, side):

        self.side_a = side

        self.side_c = RightTriangle.__compute_hypotenuse(self.side_a,
                                                         self.side_b)

    @property
    def height(self):
        return self.side_b  # I caught a missing return because of unit testing

    @height.setter
    def height(self, side):
        self.side_b = side

        self.side_c = RightTriangle.__compute_hypotenuse(self.side_a,
                                                         self.side_b)

    @property
    def hypotenuse(self):
        return self.side_c

    def area(self):
        """
        Compute the area using
        @f$ Area = \frac{1}{2}*base*height @f$

        :return: the area
        """

        return (RightTriangle.ONE_HALF * self.side_a * self.side_b)

    def __deepcopy__(self, memo):
        """
        Return a new duplicate Shape
        """

        return RightTriangle(copy.deepcopy(self.side_a),
                             copy.deepcopy(self.side_b))

    def __str__(self):
        """
        Generate a string representing the Right Triangle
        """

        return (super(Triangle, self).__str__()
                + FPT_FMT.format("Base", self.side_a)
                + FPT_FMT.format("Height", self.side_b)
                + FPT_FMT.format("Hypotenuse", self.side_c)
                + FPT_FMT.format("Perimeter", self.perimeter())
                + FPT_FMT.format("Area", self.area()))
