from hamcrest import *
import unittest

from shapes.Shape import Shape

import copy


class TestShape(unittest.TestCase):
    """
    1 - Does this piece of code perform the operations
        it was designed to perform?

    2 - Does this piece of code do something it was not
        designed to perform?

    1 Test per mutator

if __name__ == "__main__":
    s1 = Shape()
    s2 = Shape("TestShape")

    print(s1)
    print(s2)

    s1.name = "Test Set/Get name"
    print(s1.name)
    print(s1)
    """

    def setUp(self):
        self.generic_shape = Shape()
        self.original_shape = Shape("An Original Shape Name!")

    def test_default_constructor(self):
        assert_that(self.generic_shape.name, equal_to("Shape"))

    def test_constructor(self):
        assert_that(self.original_shape.name,
                    equal_to("An Original Shape Name!"))

    def test_name_setter(self):
        a_shape = Shape()
        a_shape.name = "Dodecagon"

        assert_that(a_shape.name, equal_to("Dodecagon"))

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
        original_str = str(self.original_shape)

        assert_that(original_str, starts_with("Name"))
        assert_that(original_str, contains_string("An Original Shape Name!"))
        assert_that(original_str, ends_with("\n"))

        assert_that(original_str,
                    matches_regexp("Name\\s*:\\s*An Original Shape Name!\\n"))



