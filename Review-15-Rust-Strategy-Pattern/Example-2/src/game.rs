use std::fmt;

use crate::board::Board;
use crate::player::Player;
use crate::referee::Referee;

#[derive(Clone, Debug, Default)]
pub struct Game;

#[derive(Debug)]
pub struct GameWithOnePlayerSet<'game> {
    player_1: Player<'game>,
}

#[derive(Debug)]
pub struct InitializedGame<'game> {
    player_1: Player<'game>,
    player_2: Player<'game>,

    board: Board,
}

#[derive(Debug)]
pub enum EndState {
    Win,
    Stalemate,
}

#[derive(Debug)]
pub struct CompletedGame<'game> {
    pub winner: Option<Player<'game>>,
    pub loser: Option<Player<'game>>,
    pub end_state: EndState,
}

impl Game {
    pub fn new() -> Self {
        Self
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
        println!("{}", board);
        println!();

        loop {
            let selected_move = player.next_move();

            if Referee::selected_cell_is_empty(selected_move, &board) {
                let _ = board.set_cell(selected_move, symbol);
                break;
            }
        }

        Referee::check_for_win(&board)
    }

    pub fn play_match(mut self) -> CompletedGame<'game> {
        while self.is_not_over() {
            if Self::do_one_turn(&mut self.board, &mut self.player_1, 'X') {
                println!("{}", self.board);
                println!();

                return CompletedGame {
                    winner: Some(self.player_1),
                    loser: Some(self.player_2),
                    end_state: EndState::Win,
                };
            }

            if Self::do_one_turn(&mut self.board, &mut self.player_2, 'O') {
                println!("{}", self.board);
                println!();

                return CompletedGame {
                    winner: Some(self.player_2),
                    loser: Some(self.player_1),
                    end_state: EndState::Win,
                };
            }
        }

        CompletedGame {
            winner: None,
            loser: None,
            end_state: EndState::Stalemate,
        }
    }

    pub fn is_over(&self) -> bool {
        false
    }

    pub fn is_not_over(&self) -> bool {
        !self.is_over()
    }
}

impl<'game> CompletedGame<'game> {
    pub fn is_over(&self) -> bool {
        true
    }

    pub fn is_not_over(&self) -> bool {
        !self.is_over()
    }
}

impl<'game> fmt::Display for CompletedGame<'game> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.end_state {
            EndState::Win => {
                writeln!(
                    f,
                    "Congratulations {}!",
                    self.winner
                        .as_ref()
                        .expect("winner should be set...")
                        .get_name()
                )
            }
            EndState::Stalemate => {
                writeln!(f, "Stalemate...")
            }
        }
    }
}
