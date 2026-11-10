#[derive(Clone, Debug)]
pub enum ResponseHighOrLow {
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

    fn get_guess_range(&self) -> std::ops::RangeInclusive<u32>;

    fn ask_if_number(&mut self, selector: &impl NumberSelector) -> ResponseYesOrNo {
        let guess = Self::pick_next_guess(self.get_guess_range());

        match selector.answer_yes_or_no(guess) {
            ResponseYesOrNo::Yes => {
                self.announce_win();

                ResponseYesOrNo::Yes
            }
            ResponseYesOrNo::No => {
                let response = self.ask_if_too_high_or_low(selector);

                self.update_range(response);

                ResponseYesOrNo::No
            }
        }
    }

    fn ask_if_too_high_or_low(&mut self, selector: &impl NumberSelector) -> ResponseHighOrLow {
        let guess = Self::pick_next_guess(self.get_guess_range());

        let response = selector.answer_too_high_or_low(guess);

        response
    }

    fn update_range(&mut self, response: ResponseHighOrLow);

    fn announce_win(&self);
}

pub trait NumberSelector {
    fn answer_yes_or_no(&self, guess: u32) -> ResponseYesOrNo;

    fn answer_too_high_or_low(&self, guess: u32) -> ResponseHighOrLow;
}
