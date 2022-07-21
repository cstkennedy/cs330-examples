from hamcrest import *
import unittest

from shapes.shape import Shape
import shapes.shape_factory as shape_factory

import copy


class TestShapeFactory(unittest.TestCase):
    """
    **This is technically a set of Integration Tests**

    1 - Does this piece of code perform the operations
        it was designed to perform?

    2 - Does this piece of code do something it was not
        designed to perform?

    1 Test per mutator
    """

    def setUp(self):
        self.known_shapes = copy.deepcopy(shape_factory._KNOWN_SHAPES)
        self.number_known = len(self.known_shapes)

        self.known_names = ["Circle",
                            "Square",
                            "Triangle",
                            "Equilateral Triangle",
                            "Right Triangle"]

    def test_create_shape_success(self):
        """
        Create a known valid Shape
        """

        assert_that(shape_factory.create("Circle"), is_not(None))

    def test_create_shape_failure(self):
        """
        Try to create a known invalid Shape
        """

        assert_that(shape_factory.create("Lol Nope"), is_(None))

    def test_is_known_success(self):
        """
        Create a known valid Shape
        """

        assert_that(shape_factory.is_known("Circle"), is_(True))

    def test_is_known_failure(self):
        """
        Try to create a known invalid Shape
        """

        assert_that(shape_factory.is_known("Lol Nope"), is_(False))

    def test_list_known(self):
        known_str = shape_factory.list_known()

        for name in self.known_names:
            assert_that(known_str, contains_string(name))

    def test_number_known(self):
        assert_that(shape_factory.number_known(), equal_to(self.number_known))
