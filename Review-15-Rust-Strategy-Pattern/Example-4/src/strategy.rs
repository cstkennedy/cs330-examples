use std::fmt::Debug;
use std::io::Write;
use std::io::{stdin, stdout};

pub trait Strategy: Debug {
    fn next_move(&mut self) -> usize;
}

const PROMPT_MSG: &str = "Enter your desired move (1-9): ";

#[derive(Debug)]
pub struct KeyboardStrategy<'a> {
    name: &'a str,
}

impl<'a> KeyboardStrategy<'a> {
    pub fn new(player_name: &'a str) -> Self {
        KeyboardStrategy { name: player_name }
    }

    /// Read a line from std in and trim the trailing newline.
    fn read_line_as_string() -> String {
        let mut str_buffer = String::new();
        let _ = stdin().read_line(&mut str_buffer).unwrap();
        let str_buffer = str_buffer.trim().to_string();

        str_buffer
    }
}

impl<'a> Strategy for KeyboardStrategy<'a> {
    fn next_move(&mut self) -> usize {
        print!("{}, {PROMPT_MSG}", &self.name);
        let _ = stdout().flush();

        // TODO: This should have proper error checking
        let choice = Self::read_line_as_string().parse().unwrap();

        choice
    }
}

#[derive(Debug)]
pub struct PredefinedMoves<'a> {
    my_moves: &'a [usize],
    move_idx: usize,
}

impl<'a> PredefinedMoves<'a> {
    pub fn new(some_moves: &'a [usize]) -> Self {
        PredefinedMoves {
            my_moves: some_moves,
            move_idx: 0,
        }
    }
}

impl<'a> Strategy for PredefinedMoves<'a> {
    fn next_move(&mut self) -> usize {
        let my_next_move = self.my_moves[self.move_idx];
        self.move_idx += 1;

        my_next_move
    }
}
