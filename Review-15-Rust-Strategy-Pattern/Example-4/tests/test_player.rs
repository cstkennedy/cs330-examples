#[cfg(test)]
use hamcrest2::prelude::*;
use rstest::*;

use tictactoe::player::DEFAULT_NAME;
use tictactoe::player::Player;
use tictactoe::strategy::*;


#[fixture]
fn a_player<'a>() -> Player<'a> {
    Player::builder()
        .with_name("Jay")
        .with_strategy(PredefinedMoves::new(&[5, 1, 3, 7, 9, 2, 4, 6, 8]))
        .build()
}

#[rstest]
pub fn test_is_generic(a_player: Player) {
    let player = a_player;
    assert_that!(Player::is_generic(&player), is(not(true)));

    let player = Player::builder()
        .human()
        .with_name(DEFAULT_NAME)
        .build();
    assert_that!(Player::is_generic(&player), is(true));
}

#[rstest]
pub fn test_builder_computer(a_player: Player) {
    let player = a_player;

    assert_that!(player.get_name(), is(equal_to("Jay")));
    assert_that!(player.is_human(), is(not(true)));
    assert_that!(player.is_computer(), is(true));

    assert_that!(Player::is_generic(&player), is(not(true)));
}

#[rstest]
pub fn test_next_move(a_player: Player) {
    let mut player = a_player;

    assert_that!(player.next_move(), is(equal_to(5 as usize)));
    assert_that!(player.next_move(), is(equal_to(1 as usize)));
    assert_that!(player.next_move(), is(equal_to(3 as usize)));
}

#[rstest]
pub fn test_builder_human() {
    let player = Player::builder()
        .human()
        .with_name("Tom")
        .build();

    assert_that!(player.get_name(), is(equal_to("Tom")));
    assert_that!(player.is_human(), is(true));
    assert_that!(player.is_computer(), is(not(true)));

    assert_that!(Player::is_generic(&player), is(not(true)));
}



