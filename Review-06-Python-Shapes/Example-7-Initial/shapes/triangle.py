import copy
import math

from shapes.shape import (Shape, FPT_FMT, STR_FMT)


class Triangle(Shape):
    """
    A Polygon with 3 Sides
    """

    def __init__(self, a=1, b=1, c=1):

        self._side_a = a
        self._side_b = b
        self._side_c = c

    @property
    def name(self) -> str:
        """
        Provide read-only access to the name attribute.
        """

        return "Triangle"

    @property
    def side_a(self):
        return self._side_a

    @side_a.setter
    def side_a(self, side):
        self._side_a = side

    @property
    def side_b(self):
        return self._side_b

    @side_b.setter
    def side_b(self, some_value):
        self._side_b = some_value

    @property
    def side_c(self):
        return self._side_c

    @side_c.setter
    def side_c(self, some_value):
        self._side_c = some_value

    def area(self):
        """
        Compute the area using Heron's Formula. Use

        .. math:: s = \\frac{1}{2}Perimeter

        and

        .. math:: Area = \\sqrt{ s(s-a)(s-b)(s-c) }
        """

        semi_perimeter = self.perimeter() / 2.0

        return (semi_perimeter
                * (semi_perimeter - self.side_a)
                * (semi_perimeter - self.side_b)
                * (semi_perimeter - self.side_c)) ** 0.5

    def perimeter(self):
        """
        Compute the perimeter
        """

        return self.side_a + self.side_b + self.side_c

    def __deepcopy__(self, memo):
        """
        Return a new duplicate Shape
        """

        return Triangle(copy.deepcopy(self.side_a),
                        copy.deepcopy(self.side_b),
                        copy.deepcopy(self.side_c))

    def __str__(self):
        """
        Print the Triangle
        """

        return (super().__str__()
                + FPT_FMT.format("Side A", self.side_a)
                + FPT_FMT.format("Side B", self.side_b)
                + FPT_FMT.format("Side C", self.side_c)
                + FPT_FMT.format("Perimeter", self.perimeter())
                + FPT_FMT.format("Area", self.area()))

    def __repr__(self):
        a, b, c = self.side_a, self.side_b, self.side_c
        return f"Triangle(side_a={a}, side_b={b}, side_c={c})"


class EquilateralTriangle(Triangle):
    """
    A Triangle with all sides set to the same length
    """

    ROOT_3_DIV_4 = math.sqrt(3) / 4  # @f$ \frac{\sqrt{3}}{4} @f$

    def __init__(self, side=1):
        """
        Construct an Equilateral Triangle
        with all sides set to 1 by default.
        """
        super().__init__(side, side, side)

    @property
    def name(self) -> str:
        """
        Provide read-only access to the name attribute.
        """

        return "Equilateral Triangle"

    @property
    def height(self):
        """
        Compute the height using

        .. math:: height = \\frac{5}{4}side
        """

        return math.sqrt(1.25 * (self.side ** 2))

    @property
    def side(self):
        return self.side_a

    @side.setter
    def side(self, side):

        self.side_a = side
        self.side_b = side
        self.side_c = side

    def area(self):
        """
        Compute the area using

        .. math:: Area=\\frac{\\sqrt{3}}{4}side^2
        """

        return EquilateralTriangle.ROOT_3_DIV_4 * self.side * self.side

    def __deepcopy__(self, memo):
        """
        Return a new duplicate Shape
        """

        return EquilateralTriangle(copy.deepcopy(self.side))

    def __str__(self):
        """
        Print the Equilateral Triangle
        """

        return (STR_FMT.format("Name", self.name)
                + FPT_FMT.format("Side", self.side)
                + FPT_FMT.format("Height", self.height)
                + FPT_FMT.format("Perimeter", Triangle.perimeter(self))
                + FPT_FMT.format("Area", self.area()))

    def __repr__(self):
        return f"EquilateralTriangle(side={self.side})"


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

        .. math:: hypotenuse = \\sqrt{base^2 + height^2}

        Args:
            base: the base of a Right Triangle

            height: the height of a Right Triangle

        :return: hypotenuse of a right triangle
        """
        return math.sqrt((base ** 2) + (height ** 2))

    def __init__(self, base=1, height=1):
        """
        Construct a RightTriangle with base and height set to 1 by default.
        """

        self.side_a = base
        self.side_b = height
        self.side_c = RightTriangle.__compute_hypotenuse(self.side_a,
                                                         self.side_b)

    @property
    def name(self) -> str:
        """
        Provide read-only access to the name attribute.
        """

        return "Right Triangle"

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

        .. math:: Area = \\frac{1}{2}*base*height

        :return: the area
        """

        return RightTriangle.ONE_HALF * self.side_a * self.side_b

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

        return (STR_FMT.format("Name", self.name)
                + FPT_FMT.format("Base", self.side_a)
                + FPT_FMT.format("Height", self.side_b)
                + FPT_FMT.format("Hypotenuse", self.side_c)
                + FPT_FMT.format("Perimeter", self.perimeter())
                + FPT_FMT.format("Area", self.area()))

    def __repr__(self):
        return f"RightTriangle(base={self.base}, height={self.height})"
