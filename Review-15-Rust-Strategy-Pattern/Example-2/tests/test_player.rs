#[cfg(test)]
use hamcrest2::prelude::*;
use rstest::*;

use tictactoe::player::Player;
use tictactoe::strategy::*;


#[fixture]
fn a_player<'a>() -> Player<'a> {
    Player::new(
        "Jay",
        PredefinedMoves::new(&[5, 1, 3, 7, 9, 2, 4, 6, 8]),
        false,
    )
}


#[rstest]
pub fn test_constructor(a_player: Player) {
    let player = a_player;

    assert_that!(player.get_name(), is(equal_to("Jay")));
    assert_that!(player.is_human(), is(not(true)));
    assert_that!(player.is_computer(), is(true));

    assert_that!(Player::is_generic(&player), is(not(true)));
}

#[rstest]
pub fn test_is_generic(a_player: Player) {
    let player = a_player;

    assert_that!(Player::is_generic(&player), is(not(true)));
}

#[rstest]
pub fn test_next_move(a_player: Player) {
    let mut player = a_player;

    assert_that!(player.next_move(), is(equal_to(5 as usize)));
    assert_that!(player.next_move(), is(equal_to(1 as usize)));
    assert_that!(player.next_move(), is(equal_to(3 as usize)));
}
