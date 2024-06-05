from __future__ import annotations

import logging
import pprint as pp
from dataclasses import dataclass
from enum import Enum, auto
from typing import Generator, Iterator, TextIO


class NodeType(Enum):
    INTERNAL = auto()
    LEAF = auto()


@dataclass
class Node:
    content: str
    node_type: NodeType
    left: Node = None
    right: Node = None


    def __str__(self) -> str:
        return self.content


@dataclass
class Tree:
    root: Node

    def _str_helper(self, node: Node, depth: int = 0) -> str:
        indent = "  " * depth

        if node.left and node.right:
            return "\n".join(
                (
                    f"{indent}└─{node}",
                    self._str_helper(node.left, depth + 1),
                    self._str_helper(node.right, depth + 1),
                )
            )

        return f"{indent}└─{node}"

    def __str__(self) -> str:
        return self._str_helper(self.root)


def read_file(tree_file: TextIO) -> Generator[tuple[NodeType, str], None, None]:
    for line in tree_file:
        line = line.strip()

        the_type, the_content = line.split(" - ")
        logging.debug(f"{the_type = } | {the_content = }")

        if the_type == "Q":
            the_type = NodeType.INTERNAL
        elif the_type == "A":
            the_type = NodeType.LEAF
        else:
            raise ValueError("Line must start with 'Q' or 'A'")

        yield the_type, the_content


def build_recursive(node_data: Iterator[tuple[NodeType, str]]) -> Node:
    if not node_data:
        return None

    try:
        entry = next(node_data)

        the_node = Node(content=entry[1], node_type=NodeType.INTERNAL)

        if entry[0] == NodeType.INTERNAL:
            the_node = Node(content=entry[1], node_type=NodeType.INTERNAL)
            the_node.left = build_recursive(node_data)
            the_node.right = build_recursive(node_data)
        else:
            the_node = Node(content=entry[1], node_type=NodeType.LEAF)

    except StopIteration as _err:
        return None

    return the_node


def play_game(tree: Tree) -> None:
    if tree.root.node_type == NodeType.LEAF:
        print(tree.root.content)
        return

    answer = input(f"{tree.root.content}... (left or right): ")

    answer = answer.lower()
    if answer == "left" or answer == "no":
        play_game(Tree(tree.root.left))

    else:
        play_game(Tree(tree.root.right))






def main():
    logging.basicConfig(level=logging.WARNING)

    with open("tree_1.txt", "r") as tree_file:
        raw_lines = list(read_file(tree_file))

    pp.pprint(raw_lines)
    tree = Tree(root=build_recursive(iter(raw_lines)))

    #  pp.pprint(tree, compact=False, indent=4, width=10)
    print("-" * 72)
    print(tree)
    print("-" * 72)

    play_game(tree)

if __name__ == "__main__":
    main()
