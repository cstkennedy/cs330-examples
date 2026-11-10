pub type ResponseHighOrLow<'a> = &'a str;

pub trait NumberGuesser {
    fn pick_next_guess(range: std::ops::RangeInclusive<u32>) -> u32;

    fn ask_if_number(&self) -> u32;

    fn ask_if_too_high_or_low(&self, guess: u32);

    fn update_range(&mut self, response: ResponseHighOrLow);

    fn announce_win(&self);
}

pub trait NumberSelector {
    fn answer_yes_or_no(&self, _guess: u32) -> Result<String, std::io::Error>;

    fn answer_too_high_or_low(&self, _guess: u32) -> Result<String, std::io::Error>;
}
