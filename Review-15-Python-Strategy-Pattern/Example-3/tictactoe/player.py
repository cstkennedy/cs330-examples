import copy
from typing import Self

from .strategy import Strategy

DEFAULT_NAME: str = "I. C. Generic"
DEFAULT_SYMBOL: str = "?"


class Player:
    """
    This is more a Player interface than a Player class.
    <p>
    However, such distinctions and discussions belong in
    the OOP and Inheritance Modules
    """

    @staticmethod
    def is_generic(possible_cylon: "Player") -> bool:
        """
        Checks whether a player is a placeholder or
        an actual player.

        Args:
            possible_cylon (Player): player whose humanity is in question

        Returns:
            True if the player is a Cylon
        """

        return possible_cylon == REFERENCE_CYLON

    def __init__(self, *, name: str = DEFAULT_NAME, strategy: Strategy, humanity: bool = False):
        """
        Create a Player with a selected name.

        Args:
            n: desired name
        """

        self._name = name
        self._strategy = strategy
        self._symbol = DEFAULT_SYMBOL
        self._is_human = humanity

    def get_name(self) -> str:
        """
        Retrieve name.

        Returns:
            player name
        """

        return self._name

    def set_name(self, nme: str):
        """
        Set player name.

        @param n new name

        @pre (n.size() > 0)
        """

        self._name = nme

    def next_move(self) -> int:
        """
        Retrieve the next move.

        @return board cell id representing the selected move

        @throws IOException if the move can not be retreived from the player.
        """

        return self._strategy.next_move()

    def is_human(self) -> bool:
        """
        Is this a Human Player?

        In this discussion, always yes :(

        Returns:
            True if the player is a human
        """

        return self._is_human

    def is_computer(self) -> bool:
        """
        Is this a Computer Player?

        In this discussion, always no :(

        Returns:
            True if the player is a Cylon
        """

        return not self.is_human()

    def get_symbol(self) -> str:
        """
        Retrieve player symbol to be used
        for marking moves.

        Returns:
            current player symbol
        """

        return self._symbol

    def set_symbol(self, new_symbol: str):
        """
        Change the player symbol.

        Args:
            new_symbol: new character to be used by the player
        """

        self._symbol = new_symbol

    def __eq__(self, rhs):
        if not isinstance(rhs, self.__class__):
            return False

        return self._name == rhs._name

    def __hash__(self):
        return hash(self._name)

    def __str__(self):
        """
        Generate a player string, but only the name.
        """

        return self._name

    def __deepcopy__(self, memo):
        """
        Create a new duplicate Player.
        """

        cpy = Player(name=self._name, strategy=copy.deepcopy(self._strategy))
        cpy.set_symbol(self._symbol)

        return cpy


REFERENCE_CYLON = Player(strategy=None)
"""
A Player that serves as a sentinal value or placeholder.
"""
