use eyre::{self, WrapErr};

use tictactoe::prelude::*;
use tictactoe::strategy::PredefinedMoves;

fn main() -> eyre::Result<()> {
    /*
    #[rustfmt::skip]
    let finished_game = Game::new()
        .add_player(
            Player::builder()
                .human()
                .with_name("Thomas")
                .build()
        )
        .add_player(
            Player::builder()
                .with_name("Jay")
                .with_strategy(
                    PredefinedMoves::from_iterable_nondiscarding([5, 1, 3, 7, 9, 2, 4, 6, 8])
                        .wrap_err("An invalid predefined move was supplied")?
                )
                .build(),
        )
        .play_match();
    */

    /*
    #[rustfmt::skip]
    let finished_game = Game::new_with_players(
        Player::builder()
            .human()
            .with_name("Thomas")
            .build(),
        Player::builder()
            .with_name("Jay")
            .with_strategy(
                PredefinedMoves::from_iterable_nondiscarding([5, 1, 3, 7, 9, 2, 4, 6, 8])
                    .wrap_err("An invalid predefined move was supplied")?
            )
            .build()
    ).play_match();
    */

    /*
    #[rustfmt::skip]
    let finished_game = Game::new_with_players(
        Player::create_human("Thomas"),
        Player::builder()
            .with_name("Jay")
            .with_strategy(
                PredefinedMoves::from_iterable_nondiscarding([5, 1, 3, 7, 9, 2, 4, 6, 8])
                    .wrap_err("An invalid predefined move was supplied")?
            )
            .build()
    ).play_match();
    */

    #[rustfmt::skip]
    let finished_game = Game::new_with_players(
        Player::create_human("Thomas"),
        Player::create_computer(
            "Jay",
            PredefinedMoves::from_iterable_nondiscarding([5, 1, 3, 7, 9, 2, 4, 6, 8])
                .wrap_err("An invalid predefined move was supplied")?
        )
    ).play_match();

    println!("{}", finished_game);

    Ok(())
}
