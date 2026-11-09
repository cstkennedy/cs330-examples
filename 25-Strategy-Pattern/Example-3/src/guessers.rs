use std::io::Write;
use std::io::stdout;

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

    fn ask_if_number(&mut self, selector: &impl NumberSelector) -> ResponseYesOrNo {
        let guess = Self::pick_next_guess(self.range.clone());

        print!("Is your number {guess}? (y/n): ");
        let _ = stdout().flush();

        match selector.answer_yes_or_no(guess) {
            ResponseYesOrNo::Yes => {
                self.announce_win();

                ResponseYesOrNo::Yes
            },
            ResponseYesOrNo::No => {
                // Bug - Original code double prompted.
                // self.ask_if_too_high_or_low(selector);
                // let response = selector.answer_too_high_or_low(guess);

                let response = self.ask_if_too_high_or_low(selector);

                self.update_range(response);

                ResponseYesOrNo::No
            }
        }
    }

    fn ask_if_too_high_or_low(&mut self, selector: &impl NumberSelector) -> ResponseHighOrLow{
        let guess = Self::pick_next_guess(self.range.clone());

        print!("Was {guess} too high or too low? (h/l): ");
        let _ = stdout().flush();

        let response = selector.answer_too_high_or_low(guess);

        // Bug - Original code double updated - REMOVE.
        // self.update_range(response.clone());

        response
    }

    fn update_range(&mut self, response: ResponseHighOrLow) {
        let guess = Self::pick_next_guess(self.range.clone());

        self.range = match response {
            ResponseHighOrLow::TooHigh => std::ops::RangeInclusive::new(*self.range.start(), guess),
            ResponseHighOrLow::TooLow => std::ops::RangeInclusive::new(guess, *self.range.end()),
        };

        dbg!(&self.range);
    }

    fn announce_win(&self) {
        println!("I win!");
    }
}
