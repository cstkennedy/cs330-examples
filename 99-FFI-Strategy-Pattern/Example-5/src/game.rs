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
pub struct Game<P1, P2> {
    player_1: P1,
    player_2: P2,
    board: Board,
}

impl Game<Player1NotSet, Player2NotSet> {
    pub fn new() -> Self {
        Game::default()
    }

    pub fn add_player(self, player: Player) -> Game<Player, Player2NotSet> {
        Game {
            player_1: player,
            player_2: self.player_2,
            board: self.board,
        }
    }
}

impl Game<Player, Player2NotSet> {
    pub fn add_player(self, player: Player) -> Game<Player, Player> {
        Game {
            player_1: self.player_1,
            player_2: player,
            board: self.board,
        }
    }
}

impl Game<Player, Player> {
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

    pub fn play_match(mut self) -> CompletedGame {
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

impl GameIsOver for Game<Player, Player> {
    fn is_over(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub enum CompletedGame {
    Win { winner: Player, loser: Player },
    Stalemate { player_1: Player, player_2: Player },
    Forfeit { winner: Player, loser: Player },
}

impl GameIsOver for CompletedGame {
    fn is_over(&self) -> bool {
        true
    }
}

impl fmt::Display for CompletedGame {
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
