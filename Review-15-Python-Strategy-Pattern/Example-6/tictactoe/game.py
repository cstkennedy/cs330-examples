"""
This module handles the top-level game logic, including...

1. Player symbol assignment
2. Setting up the Referee
3. Tracking match progress
4. Managing player move order and validation
"""

from dataclasses import dataclass
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
    OVER_WITH_FORFEIT = auto()


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

        self.state = GameState.NOT_STARTED

    def play_match(self) -> None:
        if not self.ready_to_start():
            raise GameStateError("Player 1 *and** player 2 must be added")

        self.state = GameState.IN_PROGRESS

        """
        while self.state == GameState.IN_PROGRESS:
            self.player_turn(self._player1, "X")

            if self.state != GameState.IN_PROGRESS:
                break

            self.player_turn(self._player2, "O")
        """

        while self.state == GameState.IN_PROGRESS:
            players_and_symbols = ((self._player1, "X"), (self._player2, "O"))

            for player, symbol in players_and_symbols:
                if self.state != GameState.IN_PROGRESS:
                    break

                self.player_turn(player, symbol)

    def player_turn(self, player: Player, symbol: str) -> None:
        """
        Play one round of Tic-Tac-Toe.

        Returns:
            True if the game ended during the turn

        Raises:
            IOException if there is an error reading the selected move

        """
        # The game ended already
        if not self.ready_to_start() and self.state != GameState.IN_PROGRESS:
            raise GameStateError()  # TODO: Add a test for this exception

        self.state = GameState.IN_PROGRESS

        player.get_render_preference().render(self._board)

        self._handle_move(player, symbol)

        if winner_symbol := self._ref.check_for_win():
            self.state = GameState.OVER_WITH_WIN

            self._winner = (
                self._player1 if winner_symbol == "X" else self._player2
            )
            self._loser = (
                self._player2 if winner_symbol == "X" else self._player1
            )

        elif self._board.is_full():
            self.state = GameState.OVER_WITH_STALEMATE

            player.get_render_preference().render(self._board)

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

    def get_board(self) -> Board:
        return self._board

    def get_winner(self) -> Optional[Player]:
        return self._winner

    def get_loser(self) -> Optional[Player]:
        return self._loser

    # --------------------------------------------------------------------------
    # Game State Examination
    # --------------------------------------------------------------------------

    def ready_to_start(self) -> bool:
        if self._player1 is None or self._player2 is None:
            return False

        return True

    def not_ready_to_start(self) -> bool:
        return not self.ready_to_start()

    def ended_with_win(self) -> bool:
        return self._winner is not None

    def ended_with_loss(self) -> bool:
        return self.state == GameState.OVER_WITH_WIN

    def ended_with_stalemate(self) -> bool:
        return self.state == GameState.OVER_WITH_STALEMATE

    def is_over(self) -> bool:
        return self.ended_with_win() or self.ended_with_stalemate()

    def is_not_over(self) -> bool:
        return not self.is_over()

    def current_state(self) -> GameState:
        return self.state

    def __str__(self) -> str:
        match self.current_state():
            case GameState.OVER_WITH_WIN:
                return f"Congratulations {self._winner.name}!"

            case GameState.OVER_WITH_STALEMATE:
                return "Stalemate..."

            case GameState.OVER_WITH_FORFEIT:
                return f"{self._loser.name} forfeited."

            case _:
                pass

        return "Game is in progress."


@dataclass(kw_only=True)
class CompletedGame:
    board: Board

    winner: Optional[Player] = None
    loser: Optional[Player] = None

    state = GameState.NOT_STARTED

    def __str__(self) -> str:
        match self.state:
            case GameState.OVER_WITH_WIN:
                return f"Congratulations {self.winner.name}!"

            case GameState.OVER_WITH_STALEMATE:
                return "Stalemate..."

            case GameState.OVER_WITH_FORFEIT:
                return f"{self.loser.name} forfeited."

            case _:
                raise GameStateError()

        return "Error"
