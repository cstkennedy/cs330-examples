import copy

from collections.abc import (Iterator, Iterable)
from dataclasses import dataclass
from typing import (Any)


class LinkedList(Iterable):
    """
    The LinkedList (LL) is a wrapper for three items.
     - Head pointer
     - Tail pointer
     - Node counter (cardinality)

    Only the head pointer is necessary, the latter three items are
    included for convenience.

    In this version, the LinkedList has been converted to a proper class
    """

    @dataclass
    class Node:
        """
        The Node class is the atom--smallest base component--of a Linked List.
        An array element can viewed as its analog.

        For simplicity in this example, the Node will store an integers as
        data. In a later example, methods for generalizing this--through use of
        templates--will be utilized.

        This version makes use of the new Python data class feature.
        """

        data: Any = 0
        next: "Node" = None

    class Iterator(Iterator):
        """
        A special purpose Linked List Iterator
        """

        def __init__(self, node):
            self.__current_node = node

        def __next__(self):
            if self.__current_node is None:
                raise StopIteration

            next_val = (self.__current_node).data

            self.__current_node = (self.current_node).next

            return next_val

        @property
        def current_node(self):
            return self.__current_node

    def __init__(self):
        """
        Construct an empty Linked List
        """

        self.__head: "Node" = None
        self.__tail: "Node" = None
        self.__nodes: int = 0

    def append(self, to_add):
        """
        Add a Node at the end of the list
        """

        # Store the "to_add" data within the node
        new_node = LinkedList.Node(data=to_add)

        # Handle the case where the first node is added
        if self.__head is None:
            self.__head = new_node
            self.__tail = new_node

        else:
            (self.__tail).next = new_node
            self.__tail = new_node

        self.__nodes += 1

        # Is this line necessary?
        new_node = None

    def __len__(self):
        return self.__nodes

    def __iter__(self):
        return LinkedList.Iterator(self.__head)

    def __str__(self):
        """
        Iterate through the LinkedList and print each individual Node
        with an index.
        """

        return "\n".join([f"Node # {index:>4} - {data:>4}"
                          for index, data in enumerate(self)])
