from hamcrest import *
import unittest

from shapes.shape import Shape

import copy


class TestShape(unittest.TestCase):
    """
    1 - Does this piece of code perform the operations
        it was designed to perform?

    2 - Does this piece of code do something it was not
        designed to perform?

    1 Test per mutator
    """

    """
    def setUp(self):
        self.generic_shape = Shape()

    def test_get_name(self):
        assert_that(calling(self.generic_shape.name),
                    raises(NotImplementedError))

    def test_area(self):
        assert_that(calling(self.generic_shape.area),
                    raises(NotImplementedError))

    def test_perimeter(self):
        assert_that(calling(self.generic_shape.perimeter),
                    raises(NotImplementedError))

    def test_deep_copy(self):
        assert_that(calling(self.generic_shape.__deepcopy__).with_args(None),
                    raises(NotImplementedError))

    def test_str(self):
        assert_that(calling(self.generic_shape.__str__),
                    raises(NotImplementedError))
"""
