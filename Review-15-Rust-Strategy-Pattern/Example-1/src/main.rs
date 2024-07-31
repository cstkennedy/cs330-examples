use tictactoe::game::Game;
use tictactoe::player::Player;
use tictactoe::strategy::*;

fn main() {
    let mut game = Game::new(
        Player::new("Thomas", KeyboardStrategy::new("Thomas"), true),
        Player::new(
            "Jay",
            PredefinedMoves::new(&[5, 1, 3, 7, 9, 2, 4, 6, 8]),
            false,
        ),
    );

    game.play_match();
}
