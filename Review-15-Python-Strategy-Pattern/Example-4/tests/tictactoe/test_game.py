import pytest
from hamcrest import assert_that, equal_to, is_, none

from tictactoe import Board, Game, GameState, GameStateError, Player
from tictactoe.builders import PlayerBuilder


def test_constructor():
    empty_board = Board()

    a_game = Game()

    assert_that(a_game.get_player1(), is_(none()))
    assert_that(a_game.get_player2(), is_(none()))

    assert_that(a_game.is_over(), is_(False))

    assert_that(a_game.get_winner(), is_(none()))
    assert_that(a_game.get_loser(), is_(none()))

    # Can not test without Board.equals method
    assert_that(a_game.get_board(), equal_to(empty_board))

    assert_that(a_game.ready_to_start(), is_(False))
    assert_that(a_game.not_ready_to_start(), is_(True))

    assert_that(a_game.current_state(), is_(GameState.NOT_STARTED))


def test_constructor_with_set_players():
    tom = Player(name="Tom", strategy=None)
    a_cylon = Player(strategy=None)

    empty_board = Board()

    a_game = Game()
    a_game.set_players(tom, a_cylon)

    assert_that(a_game.get_player1(), equal_to(tom))
    assert_that(a_game.get_player2(), equal_to(a_cylon))

    assert_that(a_game.get_player1().symbol, is_("X"))
    assert_that(a_game.get_player2().symbol, is_("O"))

    assert_that(a_game.is_over(), is_(False))

    assert_that(a_game.get_winner(), is_(none()))
    assert_that(a_game.get_loser(), is_(none()))

    # Can not test without Board.equals method
    assert_that(a_game.get_board(), equal_to(empty_board))


def test_game_start_with_no_players():
    game = Game()

    assert_that(game.get_player1(), is_(none()))
    assert_that(game.get_player2(), is_(none()))

    assert_that(game.ready_to_start(), is_(False))
    assert_that(game.not_ready_to_start(), is_(True))

    assert_that(game.current_state(), is_(GameState.NOT_STARTED))

    with pytest.raises(GameStateError):
        game.play_match()


@pytest.mark.skip("cannot test")
def test_play_round():
    pass


def test_play_match_to_stalemate():
    game = Game()

    game.set_players(
        PlayerBuilder.builder()
            .with_name("Player 1")
            .with_strategy(name="SetMoves", moves=[5, 3, 4, 9, 8])
            .build(),
        PlayerBuilder.builder()
            .with_name("Player 2")
            .with_strategy(name="SetMoves", moves=[1, 7, 6, 2])
            .build(),
    )

    game.play_match()

    assert_that(game.get_player1().name, is_(equal_to("Player 1")))
    assert_that(game.get_player2().name, is_(equal_to("Player 2")))

    assert_that(game.is_over(), is_(True))
    assert_that(game.is_not_over(), is_(False))

    expected_board = Board()
    expected_board.set_cell(5, "X")
    expected_board.set_cell(1, "O")
    expected_board.set_cell(3, "X")
    expected_board.set_cell(7, "O")
    expected_board.set_cell(4, "X")
    expected_board.set_cell(6, "O")
    expected_board.set_cell(9, "X")
    expected_board.set_cell(2, "O")
    expected_board.set_cell(8, "X")

    assert_that(game.get_board(), equal_to(expected_board))

    assert_that(game.get_winner(), is_(none()))
    assert_that(game.get_loser(), is_(none()))

    assert_that(game.ended_with_win(), is_(False))
    assert_that(game.ended_with_loss(), is_(False))
    assert_that(game.ended_with_stalemate(), is_(True))

    assert_that(game.current_state(), is_(GameState.OVER_WITH_STALEMATE))


@pytest.mark.skip("cannot test")
def test_play_match_to_win_player_1():
    pass


@pytest.mark.skip("cannot test")
def test_play_match_to_win_player_2():
    pass
