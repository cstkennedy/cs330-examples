from examples.player import Player
from examples.board import Board
from examples.referee import Referee


class Game(object):
    """
    Orchestrates a single match of Tic-Tac-Toe.
    """

    def __init__(self, p1, p2):
        """
        Construct a Game setting player 1 and player 2.

        @param p1 the 'X' player
        @param p2 the 'O' player
        """

        self._player1 = p1
        self._player2 = p2

        self._board = Board()
        self._ref = Referee(self._board)

        self._player1.set_symbol('X')
        self._player2.set_symbol('O')

        self._winner = None

    def play_round(self):
        """
        Play one round of Tic-Tac-Toe.

        @return true if the game ended during the round

        @throws IOException if there is an error reading the selected move
        """

        # The game ended already - assert could be used
        if self._board.is_full():
            return True

        winner_id = 0

        print()
        print(self._board)
        self._round_turn(self._player1)

        # The game is over
        if self._board.is_full():
            print(self._board)
            return True

        winner_id = self._ref.check_for_win()

        if winner_id == 1:
            self._winner = self._player1
            return True

        print()
        print(self._board)
        self._round_turn(self._player2)

        # Final board
        print()
        print(self._board)

        winner_id = self._ref.check_for_win()

        if winner_id == 2:
            # @DISCUSS Caught missing self with with pylint unused variable
            # check, not unit tests
            # winner = self._player2

            self._winner = self._player2
            return True

        return False

    def get_player1(self):

        return self._player1

    def get_player2(self):

        return self._player2

    def get_winner(self):

        return self._winner

    def get_loser(self):
        # @discussThisInLecture
        # Caught this bug during testing
        if self.is_not_over():
            return None

        # Stalemate
        #if ended_with_stalemate(): # @DISCUSS mistake caught with pylint
        if self.ended_with_stalemate():
            # return Player::referenceCylon
            return None

        # There was a win, figure out who lost
        if self._winner == self._player1:
            return self._player2

        return self._player1

    def ended_with_win(self):

        return self._winner is not None

    def ended_with_stalemate(self):

        return self._board.is_full() and (self._winner is None)

    def is_over(self):

        return self.ended_with_win() or self.ended_with_stalemate()

    def is_not_over(self):

        return not self.is_over()

    def get_board(self):

        return self._board

    def _round_turn(self, player):
        """
        Get a player move, and update the board.
        """

        move = player.next_move()
        sym = player.get_symbol()

        # while (board.get_cell(move) != 'X' && board.get_cell(move) != 'O') {
        while not self._ref.selected_cell_is_empty(move):
            move = player.next_move()
            sym = player.get_symbol()

        self._board.set_cell(move, sym)

        # @todo add move validation
        return True