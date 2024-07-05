from room import Room


class House:
    def __init__(self, n: str = "House"):
        """
        This is the Default constructor. Start with the name set to "House" and
        an empty set of rooms (i.e., zero rooms).
        """

        self.__name: str = n
        self.__rooms: list[Room] = []

    @property
    def name(self):
        """
        Get the name using a property.

        @property must come before @...setter...
        """

        return self.__name

    @name.setter
    def name(self, nme: str):
        """
        Set the name using a setter.

        Args:
            nme: new House name
        """

        self.__name = nme

    def add_room(self, to_add: Room):
        """
        Add another room to this House.

        Args:
            to_add: new Room to add
        """

        self.__rooms.append(to_add)

    def __len__(self):
        """
        Return the number of rooms in this House.
        """

        return len(self.__rooms)

    def is_empty(self) -> bool:
        """
        Determine whether this House is empty (i.e. `self.len() == 0).
        """

        return len(self) == 0

    def flooring_cost_metrics(self) -> tuple[float, float]:
        #  total = sum(map(lambda room: room.flooring_cost(), self))
        total = sum(room.flooring_cost() for room in self)
        avg = total / len(self)

        return total, avg

    def __iter__(self):
        """
        Wrapper around `list.__iter__()`.
        """

        return iter(self.__rooms)

    def __str__(self) -> str:
        """
        This is the equivalent of overloading:
          - `operator<<` in C++
          - `toString` in Java
          - `Display::fmt` in Rust
        """

        total, avg = self.flooring_cost_metrics()

        return (
            f"--------{self.name:}--------\n"
            + "\n".join(str(room) for room in self)
            + "------------------------------\n"
            + f"Total Cost   : $ {total:.2f}\n"
            + f"Avg Room Cost: $ {avg:.2f}\n"
        )

    def __eq__(self, rhs) -> bool:
        """
        This is the equivalent of overloading:
          - `operator==` in C++
          - `equals` in Java
          - `eq` in Rust
        """

        if self.name != rhs.name:
            return False

        if len(self) != len(rhs):
            return False

        # Python handles the traversal for us!
        return self.__rooms == rhs.__rooms
