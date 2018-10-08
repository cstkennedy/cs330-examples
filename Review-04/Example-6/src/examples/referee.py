

class Referee(object):
    """
    The Meta-player that checks game status
    e.g., checks for wins, who won, if there is
    a stalemate.

    It is an implementation detail that is not
    exposed to the outside world.
    """

    def __init__(self, board):
        """
        Create the referee and allow access
        to a board through a constant reference variable.

        @param board the board to monitor
        """

        self._boardRef = board

    def checkForWin(self):
        """
        Check for a win condition.

        @return 1 if player1, or 2 if player2 won.
                A 0 indicates neither won
        """

        winner = self._checkForHorizontalWin()

        if winner != 0:
            return winner

        winner = self._checkForVerticalWin()

        if winner != 0:
            return winner

        winner = self._checkForDiagonalWin()

        if winner != 0:
            return winner

        return 0

    def selectedCellIsEmpty(self, move):
        """
        Determine whether a cell in the board has been selected
        by a player.

        @param move player candidate move

        @return true if the cell is currently empty
        """

        # return boardRef.getCell(move) != 'X' && boardRef.getCell(move) != 'O'
        return self._boardRef.getCell(move) not in ['X', 'O']

    def _checkForHorizontalWin(self):
        """
        Check each row of the board for three 'X' or three 'O'
        characters.

        @return 1 if player 1 has won, 2 if player 2 has one, or 0 if
            no one has won
        """

        triple = self._boardRef.get3Cells(1, 2, 3)

        if self._allThreeMatch(triple):
            # if they match, grab the 'X' or 'O'
            return self._playerNumFromSymbol(triple[0][1])

        triple = self._boardRef.get3Cells(4, 5, 6)

        if self._allThreeMatch(triple):
            # if they match, grab the 'X' or 'O'
            return self._playerNumFromSymbol(triple[0][1])

        triple = self._boardRef.get3Cells(7, 8, 9)

        if self._allThreeMatch(triple):
            # if they match, grab the 'X' or 'O'
            return self._playerNumFromSymbol(triple[0][1])

        return 0

    def _checkForVerticalWin(self):
        """
        Check each column of the board for three 'X' or three 'O'
        characters.

        @return 1 if player 1 has won, 2 if player 2 has one, or 0 if
            no one has won
        """

        triple = self._boardRef.get3Cells(1, 4, 7)

        if self._allThreeMatch(triple):
            # if they match, grab the 'X' or 'O'
            return self._playerNumFromSymbol(triple[0][1])

        triple = self._boardRef.get3Cells(2, 5, 8)

        if self._allThreeMatch(triple):
            # if they match, grab the 'X' or 'O'
            return self._playerNumFromSymbol(triple[0][1])

        triple = self._boardRef.get3Cells(3, 6, 9)

        if self._allThreeMatch(triple):
            # if they match, grab the 'X' or 'O'
            return self._playerNumFromSymbol(triple[0][1])

        return 0

    def _checkForDiagonalWin(self):
        """
        Check the two diagonals of the board for three 'X' or three 'O'
        characters.

        @return 1 if player 1 has won, 2 if player 2 has one, or 0 if
            no one has won
        """

        triple = self._boardRef.get3Cells(1, 5, 9)

        if self._allThreeMatch(triple):
            # if they match, grab the 'X' or 'O'
            return self._playerNumFromSymbol(triple[0][1])

        triple = self._boardRef.get3Cells(7, 5, 3)

        if self._allThreeMatch(triple):
            # if they match, grab the 'X' or 'O'
            return self._playerNumFromSymbol(triple[0][1])

        return 0

    def _allThreeMatch(self, triple):
        """
        Check for three matching symbols in the Pair-Triple.

        @param triple set of three cells to check

        @return true if all three pairs contain the same symbol
        """

        firstVal = triple[0][1]  # std::pair first and second

        numMatches = 0

        for cell_id, symbol in triple:
            if firstVal == symbol:
                numMatches += 1

        return numMatches == 3

    def _playerNumFromSymbol(self, sym):
        """
        Given an 'X' or an 'O' determine which player is using the symbol.

        @param sym symbol to check

        @return 1 for player 1 or 2 for player 2

        @pre sym == 'X' or 'O'
        """

        # if (sym == 'X')
        #     return 1

        # return 2

        # return (sym == 'X' ? 1 : 2)
        return (1 if sym == 'X' else 2)
