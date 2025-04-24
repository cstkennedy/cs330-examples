from examples.roster import Roster
from examples.student import Student

"""
Demonstrate simple Roster and Student ADTs.
"""


def main():
    """
    This is a non-trivial main function.
    """

    john = Student("John")
    tom = Student("Tom")
    jay = Student("Jay")
    oscar = Student("Oscar")

    all_students = [john, tom, jay, oscar]

    cs330 = Roster(3, "CS 330")

    for s in all_students:
        if cs330.enroll(s):
            print(f"{s} enrolled in {cs330.course_num}")

        else:
            print(f"{s} NOT enrolled in {cs330.course_num}")

    print()
    print(cs330)


if __name__ == "__main__":
    main()
