use rstest::*;
use test_case::test_matrix;

use number_guess::{
    guessers::{BisectionGuesser, HighGuesser, LowGuesser},
    traits::{NumberGuesser, NumberSelector, ResponseHighOrLow, ResponseYesOrNo},
};

struct FixedSelector {
    chosen_num: u32,
}

impl From<u32> for FixedSelector {
    fn from(chosen_num: u32) -> Self {
        Self { chosen_num }
    }
}

impl NumberSelector for FixedSelector {
    fn answer_yes_or_no(&self, guess: u32) -> ResponseYesOrNo {
        match guess == self.chosen_num {
            true => ResponseYesOrNo::Yes,
            false => ResponseYesOrNo::No,
        }
    }

    fn answer_too_high_or_low(&self, guess: u32) -> ResponseHighOrLow {
        if guess == self.chosen_num {
            unreachable!()
        }

        if guess < self.chosen_num {
            ResponseHighOrLow::TooLow
        } else {
            ResponseHighOrLow::TooHigh
        }
    }
}

#[test_matrix(0..=100)]
#[rstest]
fn test_bisection_guesser(number_to_guess: u32) {
    const MAX_ALLOWED_ATTEMPTS: u32 = 7;
    let guess_range = 0_u32..=100;

    let selector = FixedSelector::from(number_to_guess);
    let mut guesser = BisectionGuesser::from(guess_range);

    let mut guess_attempts = 0_u32;

    loop {
        match guesser.ask_if_number(&selector) {
            ResponseYesOrNo::Yes => {
                // Number Was guessed!
                break;
            }
            ResponseYesOrNo::No => {
                guess_attempts += 1;

                if guess_attempts >= MAX_ALLOWED_ATTEMPTS {
                    assert!(
                        false,
                        "'{}' not guessed in '{}' attempts",
                        number_to_guess, MAX_ALLOWED_ATTEMPTS
                    );
                }
            }
        }
    }
}

#[test_matrix(0..=100)]
#[rstest]
fn test_high_guesser(number_to_guess: u32) {
    const MAX_ALLOWED_ATTEMPTS: u32 = 10;
    let guess_range = 0_u32..=100;

    let selector = FixedSelector::from(number_to_guess);
    let mut guesser = HighGuesser::from(guess_range);

    let mut guess_attempts = 0_u32;

    loop {
        match guesser.ask_if_number(&selector) {
            ResponseYesOrNo::Yes => {
                // Number Was guessed!
                break;
            }
            ResponseYesOrNo::No => {
                guess_attempts += 1;

                if guess_attempts >= MAX_ALLOWED_ATTEMPTS {
                    assert!(
                        false,
                        "'{}' not guessed in '{}' attempts",
                        number_to_guess, MAX_ALLOWED_ATTEMPTS
                    );
                }
            }
        }
    }
}

#[test_matrix(0..=100)]
#[rstest]
fn test_low_guesser(number_to_guess: u32) {
    const MAX_ALLOWED_ATTEMPTS: u32 = 10;
    let guess_range = 0_u32..=100;

    let selector = FixedSelector::from(number_to_guess);
    let mut guesser = LowGuesser::from(guess_range);

    let mut guess_attempts = 0_u32;

    loop {
        match guesser.ask_if_number(&selector) {
            ResponseYesOrNo::Yes => {
                // Number Was guessed!
                break;
            }
            ResponseYesOrNo::No => {
                guess_attempts += 1;

                if guess_attempts >= MAX_ALLOWED_ATTEMPTS {
                    assert!(
                        false,
                        "'{}' not guessed in '{}' attempts",
                        number_to_guess, MAX_ALLOWED_ATTEMPTS
                    );
                }
            }
        }
    }
}
