import copy

from shapes.Shape import (Shape, FPT_FMT)


class Triangle(Shape):
    """
    A Polygon with 3 Sides
    """

    def __init__(self, a=1, b=1, c=1):
        self.name = "Triangle"

        self._side_a = a
        self._side_b = b
        self._side_c = c

    @property
    def side_a(self):
        return self._side_a

    @side_a.setter
    def side_a(self, s):
        self._side_a = s

    @property
    def side_b(self):
        return self._side_b

    @side_b.setter
    def side_b(self, s):
        self._side_b = s

    @property
    def side_c(self):
        return self._side_c

    @side_c.setter
    def side_c(self, s):
        self._side_c = s

    def area(self):
        """
        Compute the area using Heron's Formula.
        Use @f$ s = \frac{1}{2}Perimeter @f$ and
        @f$ Area = \sqrt{ s(s-a)(s-b)(s-c) } @f$

        :return: area
        """

        s = self.perimeter() / 2.0

        return (s * (s - self.side_a)
                  * (s - self.side_b)
                  * (s - self.side_c)) ** 0.5

    def perimeter(self):
        """
        Compute the perimeter

        :return: perimeter
        """

        return (self.side_a + self.side_b + self.side_c)

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

        return (super(Triangle, self).__str__()
                + FPT_FMT.format("Side A", self.side_a)
                + FPT_FMT.format("Side B", self.side_b)
                + FPT_FMT.format("Side C", self.side_c)
                + FPT_FMT.format("Perimeter", self.perimeter())
                + FPT_FMT.format("Area", self.area()))

    def __repr__(self):
        return f"Triangle(side_a={self.side_a}, side_b={self.side_b}, side_c={self.side_c})"
