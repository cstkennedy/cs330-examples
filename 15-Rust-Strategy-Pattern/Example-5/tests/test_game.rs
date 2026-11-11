#[cfg(test)]
use hamcrest2::prelude::*;
use rstest::*;

use tictactoe::game::CompletedGame;
use tictactoe::prelude::*;

#[rstest]
fn test_out_of_moves_player_1() {
    let moves_for_tom: &[usize] = &[5, 1];
    let moves_for_jay: &[usize] = &[5, 1, 3, 7, 9, 2, 4, 6, 8];

    #[rustfmt::skip]
    let game = Game::new()
        .add_player(
            Player::builder()
                .with_name("Thomas")
                .with_strategy(
                    PredefinedMoves::try_from(moves_for_tom).unwrap()
                )
                .build()
        )
        .add_player(
            Player::builder()
                .with_name("Jay")
                .with_strategy(
                    PredefinedMoves::try_from(moves_for_jay).unwrap()
                )
                .build(),
        )
        .play_match();

    assert!(matches!(game, CompletedGame::Forfeit { .. }));

    /*
    assert_that!(&game.winner, is(some()));
    assert_that!(&game.loser, is(some()));
    */

    if let CompletedGame::Forfeit {
        ref winner,
        ref loser,
    } = game
    {
        assert_that!(winner.get_name(), is(equal_to("Jay")));
        assert_that!(loser.get_name(), is(equal_to("Thomas")));
    }
}

#[rstest]
fn test_out_of_moves_player_2() {
    #[rustfmt::skip]
    let game = Game::new()
        .add_player(
            Player::builder()
                .with_name("Thomas")
                .with_strategy(
                    PredefinedMoves::try_from(&[5, 1, 3, 7, 9, 2, 4, 6, 8]).unwrap()
                )
                .build()
        )
        .add_player(
            Player::builder()
                .with_name("Jay")
                .with_strategy(
                    PredefinedMoves::try_from(&[5, 1]).unwrap()
                )
                .build(),
        )
        .play_match();

    assert!(matches!(game, CompletedGame::Forfeit { .. }));

    if let CompletedGame::Forfeit {
        ref winner,
        ref loser,
    } = game
    {
        assert_that!(winner.get_name(), is(equal_to("Thomas")));
        assert_that!(loser.get_name(), is(equal_to("Jay")));
    }
}

#[rstest]
fn test_stalemate() {
    #[rustfmt::skip]
    let game = Game::new()
        .add_player(
            Player::builder()
                .with_name("Thomas")
                .with_strategy(
                    PredefinedMoves::try_from(&[5, 1, 3, 7, 9, 2, 4, 6, 8]).unwrap()
                )
                .build()
        )
        .add_player(
            Player::builder()
                .with_name("Jay")
                .with_strategy(
                    PredefinedMoves::try_from(&[5, 1, 3, 7, 9, 2, 4, 6, 8]).unwrap()
                )
                .build(),
        )
        .play_match();

    assert!(matches!(game, CompletedGame::Stalemate { .. }));

    if let CompletedGame::Stalemate {
        ref player_1,
        ref player_2,
    } = game
    {
        assert_that!(player_1.get_name(), is(equal_to("Thomas")));
        assert_that!(player_2.get_name(), is(equal_to("Jay")));
    }
}

#[rstest]
fn test_win_player_1() {
    #[rustfmt::skip]
    let game = Game::new()
        .add_player(
            Player::builder()
                .with_name("Thomas")
                .with_strategy(
                    PredefinedMoves::try_from(&[5, 1, 9]).unwrap()
                )
                .build()
        )
        .add_player(
            Player::builder()
                .with_name("Jay")
                .with_strategy(
                    PredefinedMoves::try_from(&[5, 3, 7, 9, 2, 4, 6, 8]).unwrap()
                )
                .build(),
        )
        .play_match();

    assert!(matches!(game, CompletedGame::Win { .. }));

    if let CompletedGame::Win {
        ref winner,
        ref loser,
    } = game
    {
        assert_that!(winner.get_name(), is(equal_to("Thomas")));
        assert_that!(loser.get_name(), is(equal_to("Jay")));
    }
}

#[rstest]
fn test_win_player_2() {
    #[rustfmt::skip]
    let game = Game::new()
        .add_player(
            Player::builder()
                .with_name("Thomas")
                .with_strategy(
                    PredefinedMoves::try_from(&[5, 1, 2, 4]).unwrap()
                )
                .build()
        )
        .add_player(
            Player::builder()
                .with_name("Jay")
                .with_strategy(
                    PredefinedMoves::try_from(&[3, 6, 9]).unwrap()
                )
                .build(),
        )
        .play_match();

    assert!(matches!(game, CompletedGame::Win { .. }));

    if let CompletedGame::Win {
        ref winner,
        ref loser,
    } = game
    {
        assert_that!(winner.get_name(), is(equal_to("Jay")));
        assert_that!(loser.get_name(), is(equal_to("Thomas")));
    }
}
