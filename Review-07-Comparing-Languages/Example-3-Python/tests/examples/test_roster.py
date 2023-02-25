
import copy
import unittest
from hamcrest import *

from examples.student import Student
from examples.roster import (Roster, DEFAULT_MAX_STUDENTS)


class TestRoster(unittest.TestCase):
    """
    1 - Does this piece of code perform the operations
        it was designed to perform?

    2 - Does this piece of code do something it was not
        designed to perform?

    1 Test per mutator
    """

    # Students - will not be changed during tests
    john = Student("John")
    tom = Student("Tom")
    jay = Student("Jay")
    oscar = Student("Oscar")

    # Student "lists" - will not be changed during tests
    all_students = [john, tom, jay, oscar]
    first_three_students = all_students[:3]


    def setUp(self):

        self.default_course = Roster()
        self.empty_cs350 = Roster(3, "CS 350")

    def test_default_constructor(self):

        assert_that(self.default_course.course_num, equal_to("CS 150"))
        assert_that(self.default_course.enroll_limit,
                    equal_to(DEFAULT_MAX_STUDENTS))

        assert_that(self.default_course.num_enrolled(), equal_to(0))

        # No students have been added
        assert_that(self.default_course.list_enrolled_students(), not_none())
        assert_that(len(self.default_course.list_enrolled_students()), equal_to(0))

        # skipping hashcode
        # skipping equals

        # test to_string
        assert_that(str(self.default_course), contains_string("CS 150"))
        assert_that(str(self.default_course),
                    contains_string(f"{self.default_course.num_enrolled()}"))
        assert_that(str(self.default_course),
                    contains_string(f"{DEFAULT_MAX_STUDENTS}"))


    def test_non_default_constructor(self):
        assert_that(self.empty_cs350.course_num, equal_to("CS 350"))
        assert_that(self.empty_cs350.enroll_limit, equal_to(3))

        assert_that(self.default_course.num_enrolled(), equal_to(0))

        # No students have been added
        assert_that(self.empty_cs350.list_enrolled_students(), not_none())
        assert_that(len(self.empty_cs350.list_enrolled_students()), equal_to(0))

        # NOT skipping hashcode
        assert_that(hash(self.empty_cs350), is_not(equal_to(hash(self.default_course))))
        # NOT skipping equals
        assert_that(self.empty_cs350, is_not(equal_to(self.default_course)))

        # test to_string
        assert_that(str(self.empty_cs350), contains_string("CS 350"))
        assert_that(str(self.empty_cs350),
                    contains_string(f"{self.empty_cs350.num_enrolled()}"))
        assert_that(str(self.empty_cs350), contains_string("3"))

    def test_set_course_num(self):

        cs252 = Roster()

        old_hash_code = hash(cs252)

        cs252.course_num = "CS 252"

        assert_that(cs252.course_num, equal_to("CS 252"))
        assert_that(cs252.enroll_limit,
                    equal_to(DEFAULT_MAX_STUDENTS))

        assert_that(self.default_course.num_enrolled(), equal_to(0))

        # No students have been added
        assert_that(cs252.list_enrolled_students(), not_none())
        assert_that(len(cs252.list_enrolled_students()), equal_to(0))

        # NOT skipping hashcode
        assert_that(hash(cs252), is_not(equal_to(old_hash_code)))
        # NOT skipping equals
        assert_that(cs252, is_not(equal_to(self.default_course)))

        # test to_string
        assert_that(str(cs252), contains_string("CS 252"))
        assert_that(str(cs252),
                    contains_string(f"{cs252.num_enrolled()}")) # fixed mistake
        assert_that(str(cs252), contains_string(f"{DEFAULT_MAX_STUDENTS}"))


    def test_set_enroll_limit(self):

        self.empty_cs350.enroll_limit = 2

        assert_that(self.empty_cs350.course_num, equal_to("CS 350"))
        assert_that(self.empty_cs350.enroll_limit, equal_to(2))

        assert_that(self.default_course.num_enrolled(), equal_to(0))

        # No students have been added
        assert_that(self.empty_cs350.list_enrolled_students(), not_none())
        assert_that(len(self.empty_cs350.list_enrolled_students()), equal_to(0))

        # NOT skipping hashcode
        assert_that(hash(self.empty_cs350), is_not(equal_to(hash(self.default_course))))
        # NOT skipping equals
        assert_that(self.empty_cs350, is_not(equal_to(self.default_course)))

        # test to_string
        assert_that(str(self.empty_cs350), contains_string("CS 350"))
        assert_that(str(self.empty_cs350),
                    contains_string(f"{self.empty_cs350.num_enrolled()}"))
        assert_that(str(self.empty_cs350), contains_string("2"))

    def test_enroll(self):

        #This is where the fun starts
        cs725 = Roster(3, "CS 725")

        old_hash_code = hash(cs725)

        # try to add 4 students
        assert_that(cs725.enroll(TestRoster.john), is_(True))
        assert_that(cs725.enroll(TestRoster.tom), is_(True))
        assert_that(cs725.enroll(TestRoster.jay), is_(True))
        assert_that(cs725.enroll(TestRoster.oscar), is_(False)) # should fail (limit of 3)

        assert_that(cs725.course_num, equal_to("CS 725"))
        assert_that(cs725.enroll_limit, equal_to(3))
        assert_that(cs725.num_enrolled(), equal_to(3)) # fixed mistake

        # 3 students have been added
        assert_that(cs725.list_enrolled_students(), not_none())
        assert_that(len(cs725.list_enrolled_students()), equal_to(3))
        assert_that(cs725.list_enrolled_students(),
                    contains_inanyorder(*TestRoster.first_three_students))

        assert_that(hash(cs725), is_not(equal_to(old_hash_code)))

        old_hash_code = hash(cs725)

        # Change the limit to 4
        cs725.set_enroll_limit(4) # mistake - this should be cs725
        assert_that(cs725.enroll_limit, equal_to(4)) # mistake - this should be cs725

        # try to add a 4th student with the new limit of 4
        assert_that(cs725.enroll(TestRoster.oscar), is_(True)) # should succeed (limit of 4)

        # 4 students have been added
        assert_that(cs725.list_enrolled_students(), not_none())
        assert_that(len(cs725.list_enrolled_students()), equal_to(4))
        assert_that(cs725.list_enrolled_students(),
                    contains_inanyorder(*TestRoster.all_students))

        # NOT skipping hashcode
        assert_that(hash(cs725), is_not(equal_to(old_hash_code)))
        # NOT skipping equals
        assert_that(cs725, is_not(equal_to(self.default_course)))

        # **test flaw** - did not exercise adding the same student twice
        assert_that(cs725.enroll(TestRoster.tom), is_(False)) # should fail

        # test to_string
        assert_that(str(cs725), contains_string("CS 725"))
        assert_that(str(cs725), contains_string(f"{cs725.num_enrolled()}"))
        assert_that(str(cs725), contains_string("4"))

        assert_that(str(cs725), contains_string(str(TestRoster.all_students[0])))
        assert_that(str(cs725), contains_string(str(TestRoster.all_students[1])))
        assert_that(str(cs725), contains_string(str(TestRoster.all_students[2])))
        assert_that(str(cs725), contains_string(str(TestRoster.all_students[3])))

    def test_clone(self):

        #This is where the fun continues
        cs725 = Roster(3, "CS 725")

        cs725.enroll(TestRoster.john)
        cs725.enroll(TestRoster.tom)
        cs725.enroll(TestRoster.jay)

        # Make the copy
        copy_cs725 = copy.deepcopy(cs725)

        # Both Rosters should still be identical
        assert_that(copy_cs725.course_num, equal_to(cs725.course_num))
        assert_that(copy_cs725.enroll_limit, equal_to(cs725.enroll_limit))
        assert_that(copy_cs725.num_enrolled(), equal_to(cs725.num_enrolled()))
        assert_that(hash(copy_cs725), equal_to(hash(cs725)))
        assert_that(copy_cs725, equal_to(cs725))
        assert_that(str(copy_cs725), equal_to(str(cs725)))

        # But distinct
        assert_that(copy_cs725, is_not(same_instance(cs725)))
        assert_that(copy_cs725.list_enrolled_students(),
                    is_not(same_instance(cs725.list_enrolled_students())))

        # Change the limit to 4
        copy_cs725.set_enroll_limit(4)
        assert_that(copy_cs725.enroll_limit, equal_to(4))
        assert_that(copy_cs725.enroll(TestRoster.oscar), is_(True))

        assert_that(copy_cs725.list_enrolled_students(), not_none())
        assert_that(len(copy_cs725.list_enrolled_students()), equal_to(4))
        assert_that(copy_cs725.list_enrolled_students(),
                    contains_inanyorder(*TestRoster.all_students))

        assert_that(hash(copy_cs725), is_not(equal_to(hash(cs725))))
        assert_that(copy_cs725, is_not(equal_to(self.default_course)))
        assert_that(copy_cs725, is_not(equal_to(cs725)))

        # cs725 should be unchanged
        assert_that(len(cs725.list_enrolled_students()), equal_to(3))
        assert_that(cs725.list_enrolled_students(),
                    contains_inanyorder(*TestRoster.first_three_students))

        # test to_string
        assert_that(str(copy_cs725), contains_string("CS 725"))
        assert_that(str(copy_cs725), contains_string(f"{copy_cs725.num_enrolled()}"))
        assert_that(str(copy_cs725), contains_string("4"))

        assert_that(str(copy_cs725), contains_string(str(TestRoster.all_students[0])))
        assert_that(str(copy_cs725), contains_string(str(TestRoster.all_students[1])))
        assert_that(str(copy_cs725), contains_string(str(TestRoster.all_students[2])))
        assert_that(str(copy_cs725), contains_string(str(TestRoster.all_students[3])))

        assert_that(str(copy_cs725), is_not(str(equal_to(cs725))))
