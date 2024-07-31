use crate::board::Board;
use crate::player::Player;
use crate::referee::Referee;

#[derive(Clone, Debug, Default)]
pub struct Game;

pub struct GameWithOnePlayerSet<'game> {
    player_1: Player<'game>,
}

pub struct InitializedGame<'game> {
    player_1: Player<'game>,
    player_2: Player<'game>,

    board: Board,
}

impl Game {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_player<'game>(self, player: Player<'game>) -> GameWithOnePlayerSet {
        GameWithOnePlayerSet { player_1: player }
    }
}

impl<'game> GameWithOnePlayerSet<'game> {
    pub fn add_player(self, player: Player<'game>) -> InitializedGame {
        InitializedGame {
            player_1: self.player_1,
            player_2: player,
            board: Board::new(),
        }
    }
}

impl<'game> InitializedGame<'game> {
    fn do_one_turn(board: &mut Board, player: &mut Player, symbol: char) -> bool {
        loop {
            let selected_move = player.next_move();

            if Referee::selected_cell_is_empty(selected_move, &board) {
                let _ = board.set_cell(selected_move, symbol);
                break;
            }
        }

        Referee::check_for_win(&board)
    }

    pub fn play_match(&mut self) -> bool {
        while self.is_not_over() {
            println!("{}", self.board);
            println!();
            if Self::do_one_turn(&mut self.board, &mut self.player_1, 'X') {
                print!("{}", self.board);
                println!();
                return true;
            }

            println!("{}", self.board);
            println!();
            if Self::do_one_turn(&mut self.board, &mut self.player_2, 'O') {
                print!("{}", self.board);
                println!();
                return true;
            }
        }

        false
    }

    pub fn is_over(&self) -> bool {
        self.board.is_full() || Referee::check_for_win(&self.board)
    }

    pub fn is_not_over(&self) -> bool {
        !self.is_over()
    }
}
