"""
This module handles the top-level game logic, including...

1. Player symbol assignment
2. Setting up the Referee
3. Tracking match progress
4. Managing player move order and validation
"""

from enum import StrEnum, auto
from typing import Optional

from .board import Board
from .player import Player
from .referee import Referee


class GameStateError(Exception):
    """
    This exception is raised if a game is started before both players have been
    added and their symbols set.
    """


class GameState(StrEnum):
    """
    Used to specify the current state of the game (e.g., not started, in
    progress, or complete)
    """

    NOT_STARTED = auto()
    IN_PROGRESS = auto()
    OVER_WITH_STALEMATE = auto()
    OVER_WITH_WIN = auto()


class Game:
    """
    Orchestrates a single match of Tic-Tac-Toe.
    """

    def __init__(self, player1: Player, player2: Player) -> None:
        self._board = Board()
        self._ref = Referee(self._board)

        self._player1: Player = player1
        self._player2: Player = player2

        self._winner: Optional[Player] = None
        self._loser: Optional[Player] = None

    def play_match(self) -> None:
        if not self.ready_to_start():
            #  return
            raise GameStateError("Player 1 *and** player 2 must be added")

        while self.is_not_over():
            if self.player_turn(self._player1, "X"):
                break

            if self.player_turn(self._player2, "O"):
                break

        if self.ended_with_win():
            print(f"Congratulations {self.get_winner()}!")

    def player_turn(self, player: Player, symbol: str) -> bool:
        """
        Play one round of Tic-Tac-Toe.

        Returns:
            True if the game ended during the turn

        Raises:
            IOException if there is an error reading the selected move

        """
        # The game ended already
        if self._board.is_full():
            return True

        if player.is_human():
            print()
            print(self._board)
        self._handle_move(player, symbol)

        winner_symbol = self._ref.check_for_win()

        if winner_symbol == "X":
            self._winner = self._player1
            self._loser = self._player2

            return True

        if winner_symbol == "O":
            self._winner = self._player2
            self._loser = self._player1

            return True

        # The game is over
        if self._board.is_full():
            if player.is_human():
                print(self._board)

            return True

        return False

    def _handle_move(self, player: Player, symbol: str) -> None:
        """
        Get a player move, and update the board.

        If a player provides an invalid move... keep reprompting until a valid
        move is provided.
        """
        move = player.next_move()

        while not self._ref.selected_cell_is_empty(move):
            move = player.next_move()

        self._board.set_cell(move, symbol)

    # --------------------------------------------------------------------------
    # Game Component (board and player) accessors
    # --------------------------------------------------------------------------

    def get_player1(self) -> Player:
        return self._player1

    def get_player2(self) -> Player:
        return self._player2

    def get_winner(self) -> Optional[Player]:
        return self._winner

    def get_loser(self) -> Optional[Player]:
        return self._loser

    def get_board(self) -> Board:
        return self._board

    # --------------------------------------------------------------------------
    # Game State Examination
    # --------------------------------------------------------------------------

    def ready_to_start(self) -> bool:
        # Bug... the next line returns None
        #  return self._player1 and self._player2

        if self._player1 is None or self._player2 is None:
            return False

        return True

    def not_ready_to_start(self) -> bool:
        return not self.ready_to_start()

    def ended_with_win(self) -> bool:
        return self._winner is not None

    def ended_with_loss(self) -> bool:
        return self.ended_with_win()

    def ended_with_stalemate(self) -> bool:
        return self._board.is_full() and (self._winner is None)

    def is_over(self) -> bool:
        return self.ended_with_win() or self.ended_with_stalemate()

    def is_not_over(self) -> bool:
        return not self.is_over()

    def current_state(self) -> GameState:
        if self.not_ready_to_start():
            return GameState.NOT_STARTED

        if self.ended_with_win():
            return GameState.OVER_WITH_WIN

        if self.ended_with_stalemate():
            return GameState.OVER_WITH_STALEMATE

        # The only remaining possiblity is for the game to be in progress
        return GameState.IN_PROGRESS
