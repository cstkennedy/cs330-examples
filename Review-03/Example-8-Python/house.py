from room import (Room, Flooring, DimensionSet)

from typing import List


class House:

    def __init__(self, n: str = "House"):
        """
        This is the Default constructor. Start with the name set to "House" and
        an empty set of rooms (i.e., zero rooms).
        """

        self.__name: str = n
        self.__rooms: List[House] = []


    def set_name(self, nme: str):
        """
        Set the name using a traditional (i.e., non-builder) mutator.

        # Arguments

         * `nme` - new House name
        """

        self.__name = nme


    def get_name(self):
        """
        Get the name using a traditional accessor.
        """
        return self.__name


    def add_room(self, to_add):
        """
        Add another room to this House.

        # Arguments

         * `to_add` - new Room to add
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

        total = sum(map(lambda room: room.flooring_cost(), self))
        avg = total / len(self);

        return "--------{}--------\n".format(self.__name) \
             + "\n".join([str(room) for room in self]) \
             + "------------------------------\n" \
             + f"Total Cost   : $ {total:.2f}\n" \
             + f"Avg Room Cost: $ {avg:.2f}\n"


    def __eq__(self, rhs) -> bool:
        """
        This is the equivalent of overloading:
          - `operator==` in C++
          - `equals` in Java
          - `eq` in Rust
        """

        if self.__name != rhs.__name:
            return False

        if len(self) != len(rhs):
            return False

        return self.__rooms == rhs.__rooms

        """
        # We are guaranteed to have the same number of rooms
        let mut lhs_it = self.iter();
        let mut rhs_it = rhs.iter();

        let mut lhs_room = lhs_it.next();
        let mut rhs_room = rhs_it.next();

        while lhs_room != None && rhs_room != None {
            match (lhs_room, rhs_room) {
                (Some(lhs), Some(rhs)) => {
                    if lhs != rhs {
                        return false;
                    }

                    lhs_room = lhs_it.next();
                    rhs_room = rhs_it.next();
                }
                (Some(_lhs), None) => {
                    return false;
                }
                (None, Some(_rhs)) => {
                    return false;
                }
                _ => {}
            }
        }
        """

        """
        if lhs_room == None && rhs_room == None {
            return true;
        }

        false
        """

        """
        return lhs_room == None
            && rhs_room == None
        """
