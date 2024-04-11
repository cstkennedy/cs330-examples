from typing import Never, Optional

from examples.board import Board
from examples.player import Player
from examples.referee import Referee
from examples.strategy import KeyboardStrategy, Strategy


class Game:
    """
    Orchestrates a single match of Tic-Tac-Toe.
    """

    def __init__(self):

        self._board = Board()
        self._ref = Referee(self._board)

        self._player1 = None
        self._player2 = None

        self._winner = None
        self._loser = None

    def set_players(self, player1: Player, player2: Player) -> Never:
        """
        Setting player 1 and player 2 and adjust their symbols.

        Args:
            player1: the 'X' player
            player2: the 'O' player
        """

        self._player1 = player1
        self._player2 = player2

        self._player1.set_symbol("X")
        self._player2.set_symbol("O")

    def ready_to_start(self) -> bool:
        return self._player1 and self._player2

    def play_match(self) -> Never:
        if not self.ready_to_start():
            return

        while self.is_not_over():
            self.play_round()

        print(self.get_board())

        if self.ended_with_win():
            print(f"Congratulations {self.get_winner()}!")

    def play_round(self) -> bool:
        """
        Play one round of Tic-Tac-Toe.

        Returns:
            True if the game ended during the round

        Raises:
            IOException if there is an error reading the selected move
        """

        # The game ended already
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
            self._loser = self._player2
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
            self._loser = self._player1
            return True

        return False

    def _round_turn(self, player) -> bool:
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

        return True

    def get_player1(self) -> Player:
        return self._player1

    def get_player2(self) -> Player:
        return self._player2

    def get_winner(self) -> Optional[Player]:
        return self._winner

    def get_loser(self) -> Optional[Player]:
        return self._loser

    def ended_with_win(self) -> bool:
        return self._winner is not None

    def ended_with_stalemate(self) -> bool:
        return self._board.is_full() and (self._winner is None)

    def is_over(self) -> bool:
        return self.ended_with_win() or self.ended_with_stalemate()

    def is_not_over(self) -> bool:
        return not self.is_over()

    def get_board(self) -> bool:
        return self._board
