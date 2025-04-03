"""
This module handles the top-level game logic, including...

1. Player symbol assignment
2. Setting up the Referee
3. Tracking match progress
4. Managing player move order and validation
"""

from dataclasses import dataclass, field
from enum import StrEnum, auto

from .board import Board
from .factories import MoveStrategyFactory, RenderStrategyFactory
from .player import Player
from .referee import Referee


class GameStateError(Exception):
    """
    This exception is raised if a game is started before both players have been
    added and their symbols set.
    """


class CompletedGame:
    @dataclass(kw_only=True)
    class Win:
        board: Board = field(compare=True)

        winner: Player = field(compare=True)
        loser: Player = field(compare=True)

        def __str__(self) -> str:
            return f"Congratulations {self.winner.name}!"

    @dataclass(kw_only=True)
    class Stalemate:
        board: Board = field(compare=True)

        def __str__(self) -> str:
            return "Stalemate..."

    @dataclass(kw_only=True)
    class Forfeit:
        board: Board = field(compare=True)

        winner: Player = field(compare=True)
        loser: Player = field(compare=True)

        def __str__(self) -> str:
            return f"{self.loser.name} forfeited."


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

    def __opponent_of(self, player: Player) -> Player:
        if player is self.player1:
            return self.player2

        return self.player1

    @staticmethod
    def use_defaults() -> None:
        MoveStrategyFactory.add_defaults()
        RenderStrategyFactory.add_defaults()

    def play_match(
        self,
    ) -> CompletedGame.Win | CompletedGame.Stalemate | CompletedGame.Forfeit:

        while True:
            players_and_symbols = ((self.player1, "X"), (self.player2, "O"))

            for player, symbol in players_and_symbols:
                player.get_render_preference().render(self.board)

                match self._player_turn(player, symbol):
                    case TurnResult.WIN:
                        return CompletedGame.Win(
                            board=self.board,
                            winner=player,
                            loser=self.__opponent_of(player),
                        )

                    case TurnResult.STALEMATE:
                        player.get_render_preference().render(self.board)

                        return CompletedGame.Stalemate(
                            board=self.board,
                        )

                    case TurnResult.FORFEIT:
                        raise NotImplementedError()

                    case _:
                        # the only possibility left is NOT_OVER_YET... no op
                        pass

    def _player_turn(self, player: Player, symbol: str) -> TurnResult:
        # Get a move and re-prompt if the move is invalid
        while True:
            move = player.next_move()

            if Referee.selected_cell_is_empty(self.board, move):
                break

        self.board.set_cell(move, symbol)

        if Referee.check_for_win(self.board):
            return TurnResult.WIN

        if self.board.is_full():
            return TurnResult.STALEMATE

        return TurnResult.NOT_OVER_YET
