import copy

from shapes.Shape import Shape
from shapes.Circle import Circle
from shapes.Square import Square
from shapes.Triangle import Triangle
from shapes.RightTriangle import RightTriangle
from shapes.EquilateralTriangle import EquilateralTriangle


class ShapeFactory(object):
    """
    The Shape Creating Wizard
    """

    # No more ShapePair
    # Dictionaries are the best
    # kind of data structure

    _known_shapes = {
        "Triangle": (
            Triangle(),
            lambda a, b, c: Triangle(a, b, c)
        ),
        "Right Triangle": (
            RightTriangle(),
            lambda base, height: RightTriangle(base, height)
        ),
        "Equilateral Triangle": (
            EquilateralTriangle(),
            lambda side: EquilateralTriangle(side)
        ),
        "Square": (
            Square(),
            lambda side: Square(side)
        ),
        "Circle": (
            Circle(),
            lambda radius: Circle(radius)
        )
    }  # _Dictionary_ of known shapes

    @staticmethod
    def create(name):
        """
        Create a Shape

        :param: name the shape to be created

        :return: A shape with the specified name
           or null if no matching shape is found
        """

        if name in ShapeFactory._known_shapes:
            return copy.deepcopy(
                ShapeFactory._known_shapes[name][0]
            )

        return None

    @staticmethod
    def createFromDictionary(name, values):
        """
        Create a Shape

        :param name: the shape to be created
        :param attributes: dictionary of values corresponding
            to the data needed to inialize a shape

        :return: A shape with the specified name
           or null if no matching shape is found
        """

        if name in ShapeFactory._known_shapes:
            return (
                ShapeFactory._known_shapes[name][1](**values)
            )

        return None

    @staticmethod
    def isKnown(name):
        """
        Determine whether a given shape is known

        :param: name the shape for which to query
        """

        return (name in ShapeFactory._known_shapes)

    @staticmethod
    def listKnown():
        """
        Print a list of known Shapes
        """
        return "\n".join(
            ["  " + name for name in ShapeFactory._known_shapes.keys()])

    @staticmethod
    def numberKnown():
        """
        Determine the number of known Shapes
        """

        return len(ShapeFactory._known_shapes)
