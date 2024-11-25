"""
This module handles the top-level game logic, including...

1. Player symbol assignment
2. Setting up the Referee
3. Tracking match progress
4. Managing player move order and validation
"""

from dataclasses import dataclass, field
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

    OVER_WITH_STALEMATE = auto()
    OVER_WITH_WIN = auto()
    OVER_WITH_FORFEIT = auto()


@dataclass(kw_only=True)
class CompletedGame:
    board: Board = field(compare=True)

    winner: Optional[Player] = field(default=None, compare=True)
    loser: Optional[Player] = field(default=None, compare=True)

    state: GameState = field(compare=False)

    def is_over(self) -> bool:
        return True

    def is_not_over(self) -> bool:
        return False

    def ended_with_win(self) -> bool:
        return self.state == GameState.OVER_WITH_WIN

    def ended_with_loss(self) -> bool:
        return self.state == GameState.OVER_WITH_WIN

    def ended_with_stalemate(self) -> bool:
        return self.state == GameState.OVER_WITH_STALEMATE

    def __str__(self) -> str:
        match self.state:
            case GameState.OVER_WITH_WIN:
                return f"Congratulations {self.winner.name}!"  # type: ignore

            case GameState.OVER_WITH_STALEMATE:
                return "Stalemate..."

            case GameState.OVER_WITH_FORFEIT:
                return f"{self.loser.name} forfeited."

            case _:
                # No other state makes sense
                raise GameStateError()

        return "Error"


class TurnResult(StrEnum):
    WIN = auto()
    STALEMATE = auto()
    NOT_OVER_YET = auto()
    FORFEIT = auto()


@dataclass(kw_only=True)
class Game:
    """
    Orchestrates a single match of Tic-Tac-Toe.
    """

    board: Board = field(default_factory=Board)

    player1: Player
    player2: Player

    def __post_init__(self):
        self._ref = Referee(self.board)

    def __opponent_of(self, player: Player) -> Player:
        if player is self.player1:
            return self.player2

        return self.player1

    def play_match(self) -> CompletedGame:
        if not self.ready_to_start():
            raise GameStateError("Player 1 *and** player 2 must be added")

        while True:
            players_and_symbols = ((self.player1, "X"), (self.player2, "O"))

            for player, symbol in players_and_symbols:
                match self._player_turn(player, symbol):
                    case TurnResult.WIN:
                        return CompletedGame(
                            state=GameState.OVER_WITH_WIN,
                            board=self.board,
                            winner=player,
                            loser=self.__opponent_of(player),
                        )

                    case TurnResult.STALEMATE:
                        return CompletedGame(
                            state=GameState.OVER_WITH_STALEMATE,
                            board=self.board,
                        )

                    case TurnResult.FORFEIT:
                        raise NotImplementedError()

                    case _:
                        # the only possibility left is NOT_OVER_YET... no op
                        pass

    def _player_turn(self, player: Player, symbol: str) -> TurnResult:
        player.get_render_preference().render(self.board)

        # Get a move and re-prompt if the move is invalid
        while True:
            move = player.next_move()

            if self._ref.selected_cell_is_empty(move):
                break

        self.board.set_cell(move, symbol)

        # Now that the board has been updated... check for a win
        if self._ref.check_for_win() is not None:
            return TurnResult.WIN

        if self.board.is_full():
            player.get_render_preference().render(self.board)

            return TurnResult.STALEMATE

        return TurnResult.NOT_OVER_YET

    def ready_to_start(self) -> bool:
        if self.player1 is None or self.player2 is None:
            return False

        return True

    def not_ready_to_start(self) -> bool:
        return not self.ready_to_start()

    def is_over(self) -> bool:
        return False

    def is_not_over(self) -> bool:
        return True
