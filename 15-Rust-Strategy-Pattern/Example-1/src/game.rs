use crate::board::Board;
use crate::player::Player;
use crate::referee::Referee;

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
