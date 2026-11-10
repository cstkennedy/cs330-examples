use std::io::Write;
use std::io::stdout;

use crate::traits::{NumberSelector, ResponseHighOrLow, ResponseYesOrNo};

pub(crate) mod utilities {
    use std::io::stdin;

    /// Read a line from std in and trim the trailing newline.
    pub(crate) fn read_line() -> Result<String, std::io::Error> {
        let mut str_buffer = String::new();
        let _ = stdin().read_line(&mut str_buffer)?;
        let str_buffer = str_buffer.trim().to_string();

        Ok(str_buffer)
    }
}

#[derive(Default, Debug)]
pub struct HumanSelector;

impl NumberSelector for HumanSelector {
    fn answer_yes_or_no(&self, guess: u32) -> ResponseYesOrNo {
        print!("Is your number {guess}? (y/n): ");
        let _ = stdout().flush();

        match utilities::read_line() {
            Ok(response) => match response.to_lowercase().as_ref() {
                "y" => ResponseYesOrNo::Yes,
                "n" => ResponseYesOrNo::No,
                _ => todo!(),
            },
            Err(_) => todo!(),
        }
    }

    fn answer_too_high_or_low(&self, guess: u32) -> ResponseHighOrLow {
        print!("Was {guess} too high or too low? (h/l): ");
        let _ = stdout().flush();

        match utilities::read_line() {
            Ok(response) => match response.to_lowercase().as_ref() {
                "h" => ResponseHighOrLow::TooHigh,
                "l" => ResponseHighOrLow::TooLow,
                _ => todo!(),
            },
            Err(_) => todo!(),
        }
    }
}
