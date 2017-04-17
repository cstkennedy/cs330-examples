import copy

from shapes.Shape import Shape


class Square(Shape):
    """
    A Rectangle with 4 Equal Sides
    """

    def __init__(self, side=1):
        """
        Construct a Square
        """
        self.name = "Square"

        self._side = side

    @property
    def side(self):
        return self._side

    @side.setter
    def side(self, s):
        self._side = s

    def area(self):
        """
        Compute the area

        :return: area
        """
        return self._side ** 2.0

    def perimeter(self):
        """
        Compute the perimeter

        :return: perimeter
        """

        return 4 * self._side

    def __deepcopy__(self, memo):
        """
        Return a new duplicate Shape
        """

        return Square(copy.deepcopy(self.side))

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
            super(Square, self).__str__() +
            formatStr.format("Side", self.side) +
            formatStr.format("Perimeter", self.perimeter()) +
            formatStr.format("Area", self.area())
        )


if __name__ == "__main__":
    s = Square(7)

    print(s)
