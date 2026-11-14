use std::fmt;

use crate::board::{Board, Symbol};
use crate::error::StrategyError;
use crate::player::Player;
use crate::referee::{Referee, TurnResult};

pub trait GameIsOver {
    fn is_over(&self) -> bool;

    fn is_not_over(&self) -> bool {
        GameIsOver::is_over(self)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Player1NotSet;

#[derive(Clone, Debug, Default)]
pub struct Player2NotSet;

#[derive(Clone, Debug, Default)]
pub struct NotReady;

#[derive(Clone, Debug, Default)]
pub struct InProgress;

// TODO: Remove third generic since Player state encodes the same state information.
#[derive(Clone, Debug, Default)]
pub struct Game<P1, P2, S> {
    player_1: P1,
    player_2: P2,
    board: Board,
    state: S,
}

impl Game<Player1NotSet, Player2NotSet, NotReady> {
    pub fn new() -> Self {
        Game::default()
    }

    // TODO: Move to PyGame
    pub fn new_with_players<'game>(
        player_1: Player<'game>,
        player_2: Player<'game>,
    ) -> Game<Player<'game>, Player<'game>, InProgress> {
        Game::new().add_player(player_1).add_player(player_2)
    }

    pub fn add_player(self, player: Player<'_>) -> Game<Player<'_>, Player2NotSet, NotReady> {
        Game {
            player_1: player,
            player_2: self.player_2,
            board: self.board,
            state: self.state,
        }
    }
}

impl<'game> Game<Player<'game>, Player2NotSet, NotReady> {
    pub fn add_player(
        self,
        player: Player<'game>,
    ) -> Game<Player<'game>, Player<'game>, InProgress> {
        Game {
            player_1: self.player_1,
            player_2: player,
            board: self.board,
            state: InProgress,
        }
    }
}

impl<'game> Game<Player<'game>, Player<'game>, InProgress> {
    fn do_one_turn(board: &mut Board, player: &mut Player, symbol: Symbol) -> TurnResult {
        loop {
            match player.next_move() {
                Ok(selected_move) => {
                    // if Referee::selected_cell_is_empty(selected_move, board) {
                    if board.cell_is_empty(selected_move) {
                        board.set_cell(selected_move, symbol);
                        break;
                    }
                }
                Err(StrategyError::OutOfMovesError(_)) => {
                    return TurnResult::Forfeit;
                }
                Err(_) => {}
            }
        }

        Referee::determine_turn_result(&board)
    }

    pub fn play_match(mut self) -> CompletedGame<'game> {
        loop {
            let players = vec![
                (&mut self.player_1, Symbol::X),
                (&mut self.player_2, Symbol::O),
            ];

            for (player, symbol) in players {
                println!("{}", self.board);
                println!();

                match Self::do_one_turn(&mut self.board, player, symbol) {
                    TurnResult::Win => {
                        println!("{}", self.board);
                        println!();

                        return match symbol {
                            Symbol::X => CompletedGame::Win {
                                winner: self.player_1,
                                loser: self.player_2,
                            },
                            Symbol::O => CompletedGame::Win {
                                winner: self.player_2,
                                loser: self.player_1,
                            },
                        };
                    }
                    TurnResult::Stalemate => {
                        return CompletedGame::Stalemate {
                            player_1: self.player_1,
                            player_2: self.player_2,
                        };
                    }
                    TurnResult::Forfeit => {
                        return match symbol {
                            Symbol::X => CompletedGame::Forfeit {
                                winner: self.player_2,
                                loser: self.player_1,
                            },
                            Symbol::O => CompletedGame::Forfeit {
                                winner: self.player_1,
                                loser: self.player_2,
                            },
                        };
                    }
                    TurnResult::NotOverYet => {}
                }
            }
        }
    }
}

impl GameIsOver for Game<Player<'_>, Player<'_>, InProgress> {
    fn is_over(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub enum CompletedGame<'game> {
    Win {
        winner: Player<'game>,
        loser: Player<'game>,
    },
    Stalemate {
        player_1: Player<'game>,
        player_2: Player<'game>,
    },
    Forfeit {
        winner: Player<'game>,
        loser: Player<'game>,
    },
}

impl GameIsOver for CompletedGame<'_> {
    fn is_over(&self) -> bool {
        true
    }
}

impl<'game> fmt::Display for CompletedGame<'game> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Win { ref winner, .. } => {
                writeln!(f, "Congratulations {}!", winner.get_name())
            }
            Self::Stalemate { .. } => {
                writeln!(f, "Stalemate...")
            }
            Self::Forfeit {
                winner: _, // ignore
                ref loser,
            } => {
                writeln!(f, "{} forfeited.", loser.get_name())
            }
        }
    }
}
