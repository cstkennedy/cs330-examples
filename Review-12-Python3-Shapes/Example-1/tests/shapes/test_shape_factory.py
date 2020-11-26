from hamcrest import *
import unittest

from shapes.Shape import (Shape, FPT_FMT)
from shapes.ShapeFactory import ShapeFactory

import copy


class TestShapeFactory(unittest.TestCase):
    """
    **This is technically an Integration Tests**

    1 - Does this piece of code perform the operations
        it was designed to perform?

    2 - Does this piece of code do something it was not
        designed to perform?

    1 Test per mutator
    """

    def setUp(self):
        self.known_shapes = copy.deepcopy(ShapeFactory._known_shapes)
        self.number_known = len(self.known_shapes)

    def test_create_shape_success(self):
        """
        Create a known valid Shape
        """

        assert_that(ShapeFactory.create("Circle"), is_not(None))

    def test_create_shape_failure(self):
        """
        Try to create a known invalid Shape
        """

        assert_that(ShapeFactory.create("Lol Nope"), is_(None))

    def test_is_known_success(self):
        """
        Create a known valid Shape
        """

        assert_that(ShapeFactory.is_known("Circle"), is_(True))

    def test_is_known_failure(self):
        """
        Try to create a known invalid Shape
        """

        assert_that(ShapeFactory.is_known("Lol Nope"), is_(False))

    def test_list_known(self):
        known_str = ShapeFactory.list_known()

        for pair in self.known_shapes:
            assert_that(known_str, contains_string(pair.name))

    def test_number_known(self):
        assert_that(ShapeFactory.number_known(), equal_to(self.number_known))
