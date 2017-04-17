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

    class ShapePair(object):
        """
        Name Shape Pair 2-tuple(name, model)

        I could used namedtuple instead, but...
        """

        def __init__(self, name="", shape=None):
            self.name = name  # public (no _ or __ "name-mangling")
            self.model = shape

        def __str__(self):
            return self.name

    _known_shapes = [
        ShapePair("Triangle",
                  Triangle()),
        ShapePair("Right Triangle",
                  RightTriangle()),
        ShapePair("Equilateral Triangle",
                  EquilateralTriangle()),
        ShapePair("Square",
                  Square()),
        ShapePair("Circle",
                  Circle())
    ]  # _List_ of known shapes

    @staticmethod
    def createShape(name):
        """
        Create a Shape

        :param: name the shape to be created

        :return: A shape with the specified name
           or null if no matching shape is found
        """

        for pair in ShapeFactory._known_shapes:
            if pair.name == name:
                return copy.deepcopy(pair.model)

        return None

    @staticmethod
    def isKnown(name):
        """
        Determine whether a given shape is known

        :param: name the shape for which to query
        """

        for pair in ShapeFactory._known_shapes:
            if pair.name == name:
                return True

        return False

    @staticmethod
    def listKnown():
        """
        Print a list of known Shapes
        """
        return "\n".join(
            ["  " + str(s) for s in ShapeFactory._known_shapes])

    @staticmethod
    def numberKnown():
        """
        Determine the number of known Shapes
        """

        return len(ShapeFactory._known_shapes)
