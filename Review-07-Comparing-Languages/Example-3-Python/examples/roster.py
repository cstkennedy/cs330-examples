from __future__ import annotations

import copy

from examples.student import Student

DEFAULT_MAX_STUDENTS: int = 10


class Roster:
    """
    A class roster listing all students enrolled in a course.
    """

    def __init__(self, l: int = DEFAULT_MAX_STUDENTS, c: str = "CS 150"):
        """
        Create a class roster with a specified
        enrollment limit and course number.

        Args:
            l enrollment limit
            c course number
        """
        self.course_num = c
        self.enroll_limit = l

        self.students = set()

    @property
    def course_number(self):
        return self.course_num

    @course_number.setter
    def course_number(self, n: int):
        self.course_num = n

    @property
    def get_enrollment_limit(self):
        return self.enroll_limit

    def set_enroll_limit(self, n: int):
        """
        Change the enrollment limit.
         *
        @param n desired limit
        """

        self.enroll_limit = n

    def enroll(self, stu: Student) -> bool:
        """
        Attempt to enroll a Student in the course.

        Rules:
          1- A student can not be added if the enrollment
            limit has been reached.
          2- A can not be added to a roster multiple times
         *

        Args:
          stu Student to enroll

        Returns:
           true if the Student was successfully enrolled
           in the course (added to the roster)
        """
        # limit has been reached
        if len(self.students) == self.enroll_limit:
            return False

        # Student was previously added to the roster
        if stu in self.students:
            return False

        self.students.add(stu)
        return True

    def num_enrolled(self):
        """
        Retrieve the number of enrolled students.
        """

        return len(self.students)

    def __len__(self):
        """
        Retrieve the number of enrolled students.
        """
        return len(self.students)

    def list_enrolled_students(self):
        """
        Return a collection of students in
        the order they were enrolled (added).
         *
        @return Set of enrolled students. If no students
            have been added to the roster, the set will be
            empty.
        """

        return self.students

    def __eq__(self, rhs) -> bool:
        """
        Compare 2 `Student`s based on name.
         *
        @param rhs the other (right-hand-side) student object
        """
        if not isinstance(rhs, Roster):
            return False

        if self.course_num != rhs.course_num:
            return False

        if self.enroll_limit != rhs.enroll_limit:
            return False

        if self.students != rhs.students:
            return False

        return True

    def __hash__(self):
        hc = hash(self.course_num)

        hc += self.enroll_limit

        # Python sets can not be hashed (3.11.2)
        #  hc += hash(self.students)
        hc += sum(hash(stu) for stu in self.students)

        return hc

    def __deepcopy__(self, memo):
        cpy = Roster(self.enroll_limit, self.course_num)

        # Now add the students
        for stu in self.students:
            cpy.students.add(stu)

        return cpy

    def __str__(self):
        """
        Generate a String containing the course number, number of enrolled
        students, enrollment limit, and the names of all enrolled students.
        """

        percent_full = 100.0 * len(self.students) / self.enroll_limit

        return (
            self.course_num
            + f" -> {len(self.students):>2} of {self.enroll_limit:>2}"
            + f" ({percent_full}% full)\n"
            + "\n".join((f"  - {stu}" for stu in self.students))
        )
