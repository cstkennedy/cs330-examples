use eyre::{self, WrapErr};

use tictactoe::prelude::*;
use tictactoe::strategy::PredefinedMoves;

fn main() -> eyre::Result<()> {
    let jay_s_moves: &[usize] = &[5, 1, 3, 7, 9, 2, 4, 6, 8];

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
                    PredefinedMoves::try_from(jay_s_moves)
                        .wrap_err("An invalid predefined move was supplied")?
                )
                .build(),
        )
        .play_match();

    println!("{}", finished_game);

    Ok(())
}
