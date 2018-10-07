

class Game(object):
    """
    Orchestrates a single match of Tic-Tac-Toe.
    """

    def __init__(self):
        """
        Construct a Game setting player 1 and player 2.

        @param p1 the 'X' player
        @param p2 the 'O' player
        """

        self._player1 = p1
        self._player2 = p2

        self._board = Board()
        self._ref = Referee(self._board)

        self._player1.setSymbol('X')
        self._player2.setSymbol('O')

        self._winner = None

    def playRound(self):
        """
        Play one round of Tic-Tac-Toe.

        @return true if the game ended during the round

        @throws IOException if there is an error reading the selected move
        """

        # The game ended already - assert could be used
        if self._board.isFull():
            return True

        winnerId = 0

        print(self._board)
        roundTurn(self._player1)

        # The game is over
        if self._board.isFull():
            print(self._board)
            return True

        winnerId = self._ref.checkForWin()

        if winnerId == 1:
            self._winner = self._player1
            return True

        print(self._board)
        roundTurn(self._player2)

        # Final board
        print(self._board)

        winnerId = self._ref.checkForWin()

        if winnerId == 2:
            winner = self._player2
            return True

        return False

    def getPlayer1(self):

        return self._player1

    def getPlayer2(self):

        return self._player2

    def getWinner(self):

        return self._winner

    def getLoser(self):
        # @discussThisInLecture
        # Caught this bug during testing
        if self.isNotOver():
            return None

        # Stalemate
        if endedWithStalemate():
            # return Player::referenceCylon
            return None

        # There was a win, figure out who lost
        if self._winner == self._player1:
            return self._player2

        return self._player1

    def endedWithWin(self):

        return (self._winner is not None)

    def endedWithStalemate(self):

        return self._board.isFull() and (self._winner is None)

    def isOver(self):

        return (endedWithWin() or endedWithStalemate())

    def isNotOver(self):

        return not isOver()

    def getBoard(self):

        return self._board

    def _roundTurn(self, player):
        """
        Get a player move, and update the board.
        """

        move = player.nextMove()
        sym  = player.getSymbol()

        # while (board.getCell(move) != 'X' && board.getCell(move) != 'O') {
        while not ref.selectedCellIsEmpty(move):
            move = player.nextMove()
            sym  = player.getSymbol()

        self._board.setCell(move, sym)

        # @todo add move validation
        return True
