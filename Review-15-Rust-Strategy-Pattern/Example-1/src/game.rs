use crate::board::Board;

type Player = String;

pub struct Game {
    player_1: Player,
    player_2: Player,

    board: Board,
}

impl Game {
    pub fn new(player_1: Player, player_2: Player) -> Self {
        Game {
            player_1: player_1,
            player_2: player_2,
            board: Board::new(),
        }
    }

    pub fn do_board_update(self) {
        if self.board.is_full() {
        } else {
        }
    }
}
