use std::fmt;

use crate::board::Board;
use crate::player::Player;
use crate::referee::Referee;

#[derive(Clone, Debug, Default)]
pub struct Player1NotSet;

#[derive(Clone, Debug, Default)]
pub struct Player2NotSet;

#[derive(Clone, Debug, Default)]
pub struct NotReady;

#[derive(Clone, Debug, Default)]
pub struct InProgress;

#[derive(Debug)]
pub enum EndState {
    Win,
    Stalemate,
}

#[derive(Clone, Debug, Default)]
pub struct Game<P1, P2, S> {
    player_1: P1,
    player_2: P2,
    board: Board,
    state: S
}

impl Game<Player1NotSet, Player2NotSet, NotReady> {
    pub fn new() -> Self {
        Game::default()
    }

    pub fn add_player<'game>(self, player: Player<'game>) -> Game<Player<'game>, Player2NotSet, NotReady> {
        Game {
            player_1: player,
            player_2: self.player_2,
            board: self.board,
            state: self.state
        }
    }
}

impl<'game> Game<Player<'game>, Player2NotSet, NotReady> {
    pub fn add_player(self, player: Player<'game>) -> Game<Player<'game>, Player<'game>, InProgress> {
        Game {
            player_1: self.player_1,
            player_2: player,
            board: Board::new(),
            state: InProgress
        }
    }
}

impl<'game> Game<Player<'game>, Player<'game>, InProgress> {
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

#[derive(Debug)]
pub struct CompletedGame<'game> {
    pub winner: Option<Player<'game>>,
    pub loser: Option<Player<'game>>,
    pub end_state: EndState,
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
