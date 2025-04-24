import copy

from shapes.Shape import (Shape, FPT_FMT)


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

        return (super(Square, self).__str__()
                + FPT_FMT.format("Side", self.side)
                + FPT_FMT.format("Perimeter", self.perimeter())
                + FPT_FMT.format("Area", self.area()))
