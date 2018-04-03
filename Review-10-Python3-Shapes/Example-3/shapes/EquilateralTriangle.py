import math
import copy

from shapes.Shape import Shape
from shapes.Triangle import Triangle


class EquilateralTriangle(Triangle):
    """
    A Triangle with all sides set to the same length
    """

    ROOT_3_DIV_4 = math.sqrt(3) / 4  # @f$ \frac{\sqrt{3}}{4} @f$

    def __init__(self, side=1):
        """
        Construct an EquilateralTriangle
        with all sides set to 1 by default.
        """
        self.name = "Equilateral Triangle"
        self.side = side

    @property
    def height(self):
        """
        Compute the height using
        @f$ height = \frac{5}{4}side @f$


        :return: height
        """

        return math.sqrt(
            1.25 * (self.side ** 2)
        )

    @property
    def side(self):
        return self.side_a

    @side.setter
    def side(self, s):

        self.side_a = s
        self.side_b = s
        self.side_c = s

    def area(self):
        """
        Compute the area using
        @f$ Area=\frac{\sqrt{3}}{4}side^2 @f$

        :return: the area
        """

        return (EquilateralTriangle.ROOT_3_DIV_4
                * self.side
                * self.side
                )

    def __deepcopy__(self, memo):
        """
        Return a new duplicate Shape
        """

        return EquilateralTriangle(
            copy.deepcopy(self.side)
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
            formatStr.format("Side", self.side) +
            formatStr.format("Height", self.height) +
            formatStr.format("Perimeter", Triangle.perimeter(self)) +
            formatStr.format("Area", self.area())
        )
