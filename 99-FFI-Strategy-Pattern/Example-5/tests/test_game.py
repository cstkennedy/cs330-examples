from tictactoe import Game, CompletedGame, Player


def test_out_of_moves_player_1() -> None:
    moves_for_tom: list[int] = [5, 1]
    moves_for_jay: list[int] = [5, 1, 3, 7, 9, 2, 4, 6, 8]

    game = Game(
        player1=Player.create_computer(name="Thomas", moves=moves_for_tom),
        player2=Player.create_computer(name="Jay", moves=moves_for_jay),
    ).play_match()

    """
    assert!(matches!(game, CompletedGame::Forfeit { .. }));

    if let CompletedGame::Forfeit {
        ref winner,
        ref loser,
    } = game
    {
        assert_that!(winner.get_name(), is(equal_to("Jay")));
        assert_that!(loser.get_name(), is(equal_to("Thomas")));
    }

    Ok(())
    """


def test_out_of_moves_player_2() -> None:
    game = Game(
        player1=Player.create_computer(
            name="Thomas", moves=[5, 1, 3, 7, 9, 2, 4, 6, 8]
        ),
        player2=Player.create_computer(name="Jay", moves=[5, 1]),
    ).play_match()

    """
    assert!(matches!(game, CompletedGame::Forfeit { .. }));

    if let CompletedGame::Forfeit {
        ref winner,
        ref loser,
    } = game
    {
        assert_that!(winner.get_name(), is(equal_to("Thomas")));
        assert_that!(loser.get_name(), is(equal_to("Jay")));
    }
    """


# [rstest]
def test_stalemate() -> None:
    game = Game(
        player1=Player.create_computer(
            name="Thomas", moves=[5, 1, 3, 7, 9, 2, 4, 6, 8]
        ),
        player2=Player.create_computer(
            name="Jay", moves=[5, 1, 3, 7, 9, 2, 4, 6, 8]
        ),
    ).play_match()

    """
    assert!(matches!(game, CompletedGame::Stalemate { .. }));

    if let CompletedGame::Stalemate {
        ref player_1,
        ref player_2,
    } = game
    {
        assert_that!(player_1.get_name(), is(equal_to("Thomas")));
        assert_that!(player_2.get_name(), is(equal_to("Jay")));
    }
    """


def test_win_player_1() -> None:
    game = Game(
        player1=Player.create_computer(name="Thomas", moves=[5, 1, 9]),
        player2=Player.create_computer(
            name="Jay", moves=[5, 3, 7, 9, 2, 4, 6, 8]
        ),
    ).play_match()

    """
    assert!(matches!(game, CompletedGame::Win { .. }));

    if let CompletedGame::Win {
        ref winner,
        ref loser,
    } = game
    {
        assert_that!(winner.get_name(), is(equal_to("Thomas")));
        assert_that!(loser.get_name(), is(equal_to("Jay")));
    }
    """


def test_win_player_2() -> None:
    game = Game(
        player1=Player.create_computer(name="Thomas", moves=[5, 1, 2, 4]),
        player2=Player.create_computer(name="Jay", moves=[3, 6, 9]),
    ).play_match()

    """
    assert!(matches!(game, CompletedGame::Win { .. }));

    if let CompletedGame::Win {
        ref winner,
        ref loser,
    } = game
    {
        assert_that!(winner.get_name(), is(equal_to("Jay")));
        assert_that!(loser.get_name(), is(equal_to("Thomas")));
    }
    """
