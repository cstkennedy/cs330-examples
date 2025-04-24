"""
This is an example Python Linked List that demonstrates:

  - iterators
  - Abstract Base Classes with the collections.abc module
  - deep copies with the copy module
  - data classes
  - properties
  - decorators
"""

from __future__ import annotations

import copy

import collections.abc as abc
from typing import (Any)


class LinkedList(abc.Iterable):
    """
    The LinkedList (LL) is a wrapper for three items.
     - Head pointer
     - Tail pointer
     - Node counter (cardinality)

    Only the head pointer is necessary, the latter three items are
    included for convenience.
    """

    class Node:
        """
        The Node class is the atom--smallest base component--of a Linked List.
        An array element can viewed as its analog.

        For simplicity in this example, the Node will store an integers as
        data. In a later example, methods for generalizing this--through use of
        templates--will be utilized.
        """

        def __init__(self, data=0, next=None):
            self.__data = data
            self.__next = next

    class Iterator(abc.Iterator):
        """
        A special purpose Linked List Iterator
        """

        def __init__(self, node: Node):
            self.__current_node = node

        def __next__(self):
            if self.__current_node is None:
                raise StopIteration

            next_val = (self.__current_node)._Node__data

            self.__current_node = self.__current_node._Node__next

            return next_val

    def __init__(self):
        """
        Construct an empty Linked List
        """

        self.__head: Node = None
        self.__tail: Node = None
        self.__nodes: int = 0

    def append(self, to_add: Any) -> None:
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
            (self.__tail)._Node__next = new_node
            self.__tail = new_node

        self.__nodes += 1

    def __deepcopy__(self, memo) -> LinkedList:
        clone = LinkedList()

        for datum in self:
            clone.append(copy.deepcopy(datum, memo))

        return clone

    def __len__(self) -> int:
        return self.__nodes

    def __iter__(self) -> LinkedList.Iterator:
        return LinkedList.Iterator(self.__head)

    def __str__(self) -> str:
        """
        Iterate through the LinkedList and print each individual Node
        with an index.
        """

        #  return "\n".join([f"Node # {index:>4} - {data:>4}"
                          #  for index, data in enumerate(self)])

        return "\n".join((f"Node # {index:>4} - {data:>4}"
                          for index, data in enumerate(self)))
