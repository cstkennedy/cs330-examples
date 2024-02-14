import copy

DEFAULT_NAME = "John Q. Smith"


class Student:
    def __init__(self, n: str = DEFAULT_NAME):
        self.__name = n

    @property
    def name(self):
        return self.__name

    @name.setter
    def name(self, n: str):
        self.__name = n

    def __eq__(self, rhs):
        """
        Compare 2 `Student`s based on name.
        """

        if not isinstance(rhs, Student):
            return False

        return self.__name == rhs.name

    def __hash__(self):
        return hash(self.__name)

    def __deepcopy__(self, memo):
        return Student(self.__name)

    def __str__(self):
        return self.__name

    def __repr__(self):
        return f'Student(n="{self.name}")'
