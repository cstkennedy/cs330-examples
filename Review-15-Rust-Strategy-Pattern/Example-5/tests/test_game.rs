#[cfg(test)]
use hamcrest2::prelude::*;
use rstest::*;

use tictactoe::prelude::*;
use tictactoe::game::EndState;

#[rstest]
#[should_panic]
fn test_out_of_moves_player_1() {
    #[rustfmt::skip]
    let _ = Game::new()
        .add_player(
            Player::builder()
                .with_name("Thomas")
                .with_strategy(
                    PredefinedMoves::new(&[5, 1])
                )
                .build()
        )
        .add_player(
            Player::builder()
                .with_name("Jay")
                .with_strategy(
                    PredefinedMoves::new(&[5, 1, 3, 7, 9, 2, 4, 6, 8])
                )
                .build(),
        )
        .play_match();
}

#[rstest]
#[should_panic]
fn test_out_of_moves_player_2() {
    #[rustfmt::skip]
    let _ = Game::new()
        .add_player(
            Player::builder()
                .with_name("Thomas")
                .with_strategy(
                    PredefinedMoves::new(&[5, 1, 3, 7, 9, 2, 4, 6, 8])
                )
                .build()
        )
        .add_player(
            Player::builder()
                .with_name("Jay")
                .with_strategy(
                    PredefinedMoves::new(&[5, 1])
                )
                .build(),
        )
        .play_match();
}

#[rstest]
fn test_stalemate() {
    #[rustfmt::skip]
    let game = Game::new()
        .add_player(
            Player::builder()
                .with_name("Thomas")
                .with_strategy(
                    PredefinedMoves::new(&[5, 1, 3, 7, 9, 2, 4, 6, 8])
                )
                .build()
        )
        .add_player(
            Player::builder()
                .with_name("Jay")
                .with_strategy(
                    PredefinedMoves::new(&[5, 1, 3, 7, 9, 2, 4, 6, 8])
                )
                .build(),
        )
        .play_match();

    assert_that!(&game.winner, is(none()));
    assert_that!(&game.loser, is(none()));

    assert_that!(game.end_state, is(equal_to(EndState::Stalemate)));
}

#[rstest]
fn test_win_player_1() {
    #[rustfmt::skip]
    let game = Game::new()
        .add_player(
            Player::builder()
                .with_name("Thomas")
                .with_strategy(
                    PredefinedMoves::new(&[5, 1, 9])
                )
                .build()
        )
        .add_player(
            Player::builder()
                .with_name("Jay")
                .with_strategy(
                    PredefinedMoves::new(&[5, 3, 7, 9, 2, 4, 6, 8])
                )
                .build(),
        )
        .play_match();

    assert_that!(&game.winner, is(some()));
    assert_that!(&game.loser, is(some()));

    assert_that!(game.winner.unwrap().get_name(), is(equal_to("Thomas")));
    assert_that!(game.loser.unwrap().get_name(), is(equal_to("Jay")));

    assert_that!(game.end_state, is(equal_to(EndState::Win)));
}

#[rstest]
fn test_win_player_2() {
    #[rustfmt::skip]
    let game = Game::new()
        .add_player(
            Player::builder()
                .with_name("Thomas")
                .with_strategy(
                    PredefinedMoves::new(&[5, 1, 2, 4])
                )
                .build()
        )
        .add_player(
            Player::builder()
                .with_name("Jay")
                .with_strategy(
                    PredefinedMoves::new(&[3, 6, 9])
                )
                .build(),
        )
        .play_match();

    assert_that!(&game.winner, is(some()));
    assert_that!(&game.loser, is(some()));

    assert_that!(game.winner.unwrap().get_name(), is(equal_to("Jay")));
    assert_that!(game.loser.unwrap().get_name(), is(equal_to("Thomas")));

    assert_that!(game.end_state, is(equal_to(EndState::Win)));
}
