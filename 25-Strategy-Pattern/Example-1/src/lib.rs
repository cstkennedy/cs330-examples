use std::io::Write;
use std::io::{stdin, stdout};

pub struct NumberGuesser;

impl NumberGuesser {
    pub fn get_next_guess(lower_limit: u32, upper_limit: u32) -> u32 {
        lower_limit + (upper_limit - lower_limit) / 2_u32
    }

    pub fn ask_if_number(guess: u32) {
        print!("Is your number {guess}? (y/n): ");
        let _ = stdout().flush();
    }

    pub fn ask_if_too_high_or_low(guess: u32) {
        print!("Was {guess} too high or too low? (h/l): ");
        let _ = stdout().flush();
    }
}

pub struct NumberSelector;

impl NumberSelector {
    pub fn answer_yes_or_no(_guess: u32) -> Result<String, std::io::Error> {
        Ok(read_line()?.to_lowercase())
    }

    pub fn answer_too_high_or_low(_guess: u32) -> Result<String, std::io::Error> {
        Ok(read_line()?.to_lowercase())
    }
}

/// Read a line from std in and trim the trailing newline.
fn read_line() -> Result<String, std::io::Error> {
    let mut str_buffer = String::new();
    let _ = stdin().read_line(&mut str_buffer)?;
    let str_buffer = str_buffer.trim().to_string();

    Ok(str_buffer)
}
