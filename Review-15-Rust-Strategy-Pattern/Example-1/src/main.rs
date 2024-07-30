use tictactoe::game::Game;
use tictactoe::player::Player;
use tictactoe::strategy::*;

fn main() {
    let _game = Game::new(
        Player::new(
            "Thomas",
            Box::new(&mut KeyboardStrategy::new("Thomas")),
            true
        ),
        Player::new(
            "Jay",
            Box::new(&mut PredefinedMoves::new(&[5, 1, 3, 7, 9, 2, 4, 6, 8])),
            false
        ),
    );
}
