use crate::traits::{NumberSelector, ResponseHighOrLow};

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
    fn answer_yes_or_no(&self, _guess: u32) -> Result<String, std::io::Error> {
        Ok(utilities::read_line()?.to_lowercase())
    }

    fn answer_too_high_or_low(&self, _guess: u32) -> Result<String, std::io::Error> {
        Ok(utilities::read_line()?.to_lowercase())
    }
}
