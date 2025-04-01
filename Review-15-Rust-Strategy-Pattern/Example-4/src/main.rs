use tictactoe::prelude::*;

fn main() {
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
                    PredefinedMoves::new(&[5, 1, 3, 7, 9, 2, 4, 6, 8])
                )
                .build(),
        )
        .play_match();

    println!("{}", finished_game);
}
