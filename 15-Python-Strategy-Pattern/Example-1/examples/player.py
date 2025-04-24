from examples.strategy import Strategy


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

        # print(REFERENCE_CYLON)
        return possible_cylon == REFERENCE_CYLON

    def __init__(self, n: str = "I. C. Generic"):
        """
        Create a Player with a selected name.

        Args:
            n: desired name
        """

        self._name = n
        self._symbol = "?"  # testing caught this

    def get_name(self) -> str:
        """
        Retrieve name.

        Returns:
            player name
        """

        return self._name

    def set_name(self, n: str):
        """
        Set player name.

        @param n new name

        @pre (n.size() > 0)
        """

        self._name = n

    def next_move(self, the_strategy: Strategy) -> int:
        """
        Retrieve the next move.

        @return board cell id representing the selected move

        @throws IOException if the move can not be retreived from the player.
        """

        return the_strategy.next_move()

    def is_human(self) -> bool:
        """
        Is this a Human Player?

        In this discussion, always yes :(

        Returns:
            True if the player is a human
        """

        return True

    def is_computer(self):
        """
        Is this a Computer Player?

        In this discussion, always no :(

        Returns:
            True if the player is a Cylon
        """

        return False

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

        cpy = Player(self._name)
        cpy.set_symbol(self._symbol)

        return cpy


REFERENCE_CYLON = Player()
"""
A Player that serves as a sentinal value or placeholder.
"""
