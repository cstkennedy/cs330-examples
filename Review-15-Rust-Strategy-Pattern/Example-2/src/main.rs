use tictactoe::game::Game;
use tictactoe::player::Player;
use tictactoe::strategy::*;

fn main() {
    /*
    let mut game = Game::new()
        .add_player(Player::new("Thomas", KeyboardStrategy::new("Thomas"), true))
        .add_player(Player::new(
            "Jay",
            PredefinedMoves::new(&[5, 1, 3, 7, 9, 2, 4, 6, 8]),
            false,
        ));

    let finished_game = game.play_match();
    println!("{}", finished_game);
    */

    let finished_game = Game::new()
        .add_player(Player::new("Thomas", KeyboardStrategy::new("Thomas"), true))
        .add_player(Player::new(
            "Jay",
            PredefinedMoves::new(&[5, 1, 3, 7, 9, 2, 4, 6, 8]),
            false,
        ))
        .play_match();

    println!("{}", finished_game);

    // println!("{:?}", Player::new("Thomas", KeyboardStrategy::new("Thomas"), true));
}
