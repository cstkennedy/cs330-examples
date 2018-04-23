import math
import copy

from shapes.Shape import Shape


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
        Print the Square
        """

        formatStr = (
            "{:<" +
            str(Shape.WIDTH_LABEL) +
            "}:{:>" +
            str(Shape.WIDTH_VALUE) +
            ".4f}\n"
        )

        return (
            super(Circle, self).__str__() +
            formatStr.format("Radius", self.radius) +
            formatStr.format("Diameter", self.diameter) +
            formatStr.format("Perimeter", self.perimeter()) +
            formatStr.format("Area", self.area())
        )


if __name__ == "__main__":
    s = Circle(7)

    print(s)
    print(s.diameter)
