use std::io::Write;
use std::io::stdout;

use crate::traits::{NumberGuesser, ResponseHighOrLow};

#[derive(Debug)]
pub struct BisectionGuesser {
    range: std::ops::RangeInclusive<u32>,
}

impl From<std::ops::RangeInclusive<u32>> for BisectionGuesser {
    fn from(range: std::ops::RangeInclusive<u32>) -> Self {
        Self { range }
    }
}

impl NumberGuesser for BisectionGuesser {
    fn pick_next_guess(range: std::ops::RangeInclusive<u32>) -> u32 {
        let lower_limit: u32 = *range.start();
        let upper_limit: u32 = *range.end();

        lower_limit + (upper_limit - lower_limit) / 2_u32
    }

    fn ask_if_number(&self) -> u32 {
        let guess = Self::pick_next_guess(self.range.clone());

        print!("Is your number {guess}? (y/n): ");
        let _ = stdout().flush();

        guess
    }

    fn ask_if_too_high_or_low(&self, guess: u32) {
        print!("Was {guess} too high or too low? (h/l): ");
        let _ = stdout().flush();
    }

    fn update_range(&mut self, response: ResponseHighOrLow) {
        let guess = Self::pick_next_guess(self.range.clone());

        self.range = match response {
            "h" => std::ops::RangeInclusive::new(*self.range.start(), guess),
            _ => std::ops::RangeInclusive::new(guess, *self.range.end()),
        };
    }

    fn announce_win(&self) {
        println!("I win!");
    }
}
