use crate::board::Board;
use crate::player::Player;

pub struct Game<'game> {
    player_1: Player<'game>,
    player_2: Player<'game>,

    board: Board,
}

impl<'game> Game<'game> {
    pub fn new(player_1: Player<'game>, player_2: Player<'game>) -> Self {
        Game {
            player_1: player_1,
            player_2: player_2,
            board: Board::new(),
        }
    }

}
