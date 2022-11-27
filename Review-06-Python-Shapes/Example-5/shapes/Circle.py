import math
import copy

from shapes.Shape import (Shape, FPT_FMT)


class Circle(Shape):
    """
    An Ellipse with equivalent major and minor axes
    """

    PI = math.pi  # \f$ \pi \f$
    TAU = 2 * PI  # \f$ \tau = 2\pi \f$

    def __init__(self, radius=1):
        """
        Construct a Circle
        """

        self.name = "Circle"
        self._radius = radius

    @property
    def radius(self):
        return self._radius

    @radius.setter
    def radius(self, s):
        self._radius = s

    @property
    def diameter(self):
        return 2 * self._radius

    def area(self):
        """
        Compute the area

        :return: area
        """
        return Circle.PI * (self._radius ** 2.0)

    def perimeter(self):
        """
        Compute the perimeter

        :return: perimeter
        """

        return Circle.TAU * self._radius

    def __deepcopy__(self, memo):
        """
        Return a new duplicate Shape
        """

        return Circle(copy.deepcopy(self.radius))

    def __str__(self):
        """
        Print the Circle
        """

        return (super(Circle, self).__str__()
                + FPT_FMT.format("Radius", self.radius)
                + FPT_FMT.format("Diameter", self.diameter)
                + FPT_FMT.format("Perimeter", self.perimeter())
                + FPT_FMT.format("Area", self.area()))

    def __repr__(self):
        return f"Circle(radius={self.radius})"
