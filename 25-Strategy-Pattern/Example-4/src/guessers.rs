use crate::traits::{NumberGuesser, NumberSelector, ResponseHighOrLow, ResponseYesOrNo};

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

    fn get_guess_range(&self) -> std::ops::RangeInclusive<u32> {
        self.range.clone()
    }

    fn update_range(&mut self, response: ResponseHighOrLow) {
        let guess = Self::pick_next_guess(self.range.clone());

        self.range = match response {
            ResponseHighOrLow::TooHigh => std::ops::RangeInclusive::new(*self.range.start(), guess),
            ResponseHighOrLow::TooLow => std::ops::RangeInclusive::new(guess, *self.range.end()),
        };
    }

    fn announce_win(&self) {
        println!("I win!");
    }
}

impl From<std::ops::RangeInclusive<u32>> for LowGuesser {
    fn from(range: std::ops::RangeInclusive<u32>) -> Self {
        Self { range }
    }
}

pub struct LowGuesser {
    range: std::ops::RangeInclusive<u32>,
}

impl NumberGuesser for LowGuesser {
    fn pick_next_guess(range: std::ops::RangeInclusive<u32>) -> u32 {
        let lower_limit: u32 = *range.start();
        let upper_limit: u32 = *range.end();

        let candidate_guess = lower_limit + (upper_limit - lower_limit) / 2_u32;

        if upper_limit - lower_limit > 40 {
            candidate_guess - 10
        } else {
            candidate_guess
        }
    }

    fn get_guess_range(&self) -> std::ops::RangeInclusive<u32> {
        self.range.clone()
    }

    fn update_range(&mut self, response: ResponseHighOrLow) {
        let guess = Self::pick_next_guess(self.range.clone());

        self.range = match response {
            ResponseHighOrLow::TooHigh => std::ops::RangeInclusive::new(*self.range.start(), guess),
            ResponseHighOrLow::TooLow => std::ops::RangeInclusive::new(guess, *self.range.end()),
        };
    }

    fn announce_win(&self) {
        println!("I win!");
    }
}

impl From<std::ops::RangeInclusive<u32>> for HighGuesser {
    fn from(range: std::ops::RangeInclusive<u32>) -> Self {
        Self { range }
    }
}

pub struct HighGuesser {
    range: std::ops::RangeInclusive<u32>,
}

impl NumberGuesser for HighGuesser {
    fn pick_next_guess(range: std::ops::RangeInclusive<u32>) -> u32 {
        let lower_limit: u32 = *range.start();
        let upper_limit: u32 = *range.end();

        let candidate_guess = lower_limit + (upper_limit - lower_limit) / 2_u32;

        if upper_limit - lower_limit > 32 {
            candidate_guess + 8
        } else {
            candidate_guess
        }
    }

    fn get_guess_range(&self) -> std::ops::RangeInclusive<u32> {
        self.range.clone()
    }

    fn update_range(&mut self, response: ResponseHighOrLow) {
        let guess = Self::pick_next_guess(self.range.clone());

        self.range = match response {
            ResponseHighOrLow::TooHigh => std::ops::RangeInclusive::new(*self.range.start(), guess),
            ResponseHighOrLow::TooLow => std::ops::RangeInclusive::new(guess, *self.range.end()),
        };
    }

    fn announce_win(&self) {
        println!("I win!");
    }
}
