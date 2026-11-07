use number_guess::{NumberGuesser, NumberSelector};

fn main() -> eyre::Result<()> {
    game_loop(0..=100_u32)
}

pub fn game_loop(range: std::ops::RangeInclusive<u32>) -> eyre::Result<()> {
    let mut lower_limit: u32 = *range.start();
    let mut upper_limit: u32 = *range.end();

    loop {
        let guess = NumberGuesser::get_next_guess(lower_limit, upper_limit);

        NumberGuesser::ask_if_number(guess);
        match NumberSelector::answer_yes_or_no(guess)?.as_ref() {
            "y" => {
                println!("I win!");
                break;
            }
            _ => {
                NumberGuesser::ask_if_too_high_or_low(guess);

                match NumberSelector::answer_too_high_or_low(guess)?.as_ref() {
                    "h" => {
                        upper_limit = guess;
                    }
                    _ => {
                        lower_limit = guess;
                    }
                }
            }
        }
    }

    Ok(())
}
