
class Player(object):
    """
    This is more a Player interface than a Player class.
    <p>
    However, such distinctions and discussions belong in
    the OOP and Inheritance Modules
    """

    """
    Message used to prompt a human player for a move.
    """
    PROMPT_MSG = "Enter your desired move (1-9): "

    def isGeneric(self, possibleCylon):
        """
        Checks whether a player is a placeholder or
        an actual player.

        @param possibleCylon the player whose humanity is in question

        @return true if the player is a Cylon
        """

        return possibleCylon == Player.REFERENCE_CYLON

    def __init__(self, n="I. C. Generic"):
        """
        Create a Player with a selected name.

        @param n desired name
        """

        self._name = n
        self._symbol = '?'  # testing caught this

    def getName(self):
        """
        Retrieve name.

        @return player name
        """

        return self._name

    def setName(self, n):
        """
        Set player name.

        @param n new name

        @pre (n.size() > 0)
        """

        self._name = n

    def nextMove(self):
        """
        Retrieve the next move.

        @return board cell id representing the selected move

        @throws IOException if the move can not be retreived from the player.
        """

        choice = int(input(self._name + ", " + Player.PROMPT_MSG))

        return choice

    def isHuman(self):
        """
        Is this a Human Player?
        <p>
        In this discussion, always yes :(

        @return true if the player is a human
        """

        return True

    def isComputer(self):
        """
        Is this a Computer Player?
        <p>
        In this discussion, always no :(

        @return true if the player is a Cylon
        """

        return False

    def getSymbol(self):
        """
        Retrieve player symbol to be used
        for marking moves.

        @return current player symbol
        """

        return self._symbol

    def setSymbol(self, newSymbol):
        """
        Change the player symbol.

        @param newSymbol new character to be used by the player
        """

        self._symbol = newSymbol

    def _eq_(self, rhs):
        if not isinstance(rhs, self._class_):
            return False

        return self._name == rhsP.name

    def __hash__(self):
        return hash(self.name)

    def __str__(self):
        """
        Generate a player string, but only the name.
        """

        return self._name

    def __deepcopy__(self, memo):
        """
        Return a new duplicate Player
        """

        cpy = Player(self._name)
        cpy.setSymbol(self._symbol)

        return cpy

"""
A Player that serves as a sentinal value or placeholder.
"""
REFERENCE_CYLON = Player()