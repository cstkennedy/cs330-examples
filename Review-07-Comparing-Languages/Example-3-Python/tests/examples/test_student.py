from hamcrest import *
import unittest

from examples.student import (Student, DEFAULT_NAME)


class TestStudent(unittest.TestCase):
    """
    1 - Does this piece of code perform the operations
        it was designed to perform?

    2 - Does this piece of code do something it was not
        designed to perform?

    1 Test per mutator
    """

    def setUp(self):
        self.student = Student()

    def test_default_constructor(self):
        assert_that(self.student.name, is_(DEFAULT_NAME))
        assert_that(self.student.name, equal_to(DEFAULT_NAME))

        assert_that(hash(self.student), equal_to(hash(DEFAULT_NAME)))

        assert_that(str(self.student), equal_to(DEFAULT_NAME))

    def test_non_default_constructor(self):
        desired_name = "Tommy Oliver"
        tommy = Student(desired_name)

        assert_that(tommy.name, equal_to(desired_name))

        assert_that(hash(tommy), is_not(equal_to(hash(self.student))))

        assert_that(str(tommy), equal_to(desired_name))
        assert_that(str(tommy), contains_string(desired_name))

        assert_that(tommy, is_not(equal_to(self.student)))

    def test_set_name(self):
        john = Student()
        new_name = "John T. Smith"

        old_hash_code = hash(john)

        john.name = new_name

        assert_that(john.name, equal_to(new_name))
        assert_that(hash(john), is_not(old_hash_code))

        # assert_that(str(john), equal_to(new_name))
        assert_that(str(john), contains_string(new_name))

        assert_that(john, is_not(equal_to(self.student)))

    def test_clone(self):
        self.fail("Left as an Exercise")
