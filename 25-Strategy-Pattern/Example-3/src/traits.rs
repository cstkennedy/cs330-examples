#[derive(Clone, Debug)]
pub enum ResponseHighOrLow{
    TooHigh,
    TooLow,
}

#[derive(Clone, Debug)]
pub enum ResponseYesOrNo {
    Yes,
    No,
}

pub trait NumberGuesser {
    fn pick_next_guess(range: std::ops::RangeInclusive<u32>) -> u32;

    fn ask_if_number(&mut self, selector: &impl NumberSelector) -> ResponseYesOrNo;

    fn ask_if_too_high_or_low(&mut self, selector: &impl NumberSelector) -> ResponseHighOrLow;

    fn update_range(&mut self, response: ResponseHighOrLow);

    fn announce_win(&self);
}

pub trait NumberSelector {
    fn answer_yes_or_no(&self, guess: u32) -> ResponseYesOrNo;

    fn answer_too_high_or_low(&self, guess: u32) -> ResponseHighOrLow;
}
