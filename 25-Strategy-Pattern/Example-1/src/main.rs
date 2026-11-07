use number_guess::{NumberGuesser, NumberSelector};

fn main() -> eyre::Result<()> {
    let mut lower_limit = 0_u32;
    let mut upper_limit = 100_u32;

    loop {
        let next_guess = NumberGuesser::get_next_guess(lower_limit, upper_limit);

        NumberGuesser::ask_if_number(next_guess);
        match NumberSelector::answer_yes_or_no()?.as_ref() {
            "y" => {
                println!("I win!");
                break;
            }
            _ => {
                NumberGuesser::ask_if_too_high_or_low();

                match NumberSelector::answer_too_high_or_low()?.as_ref() {
                    "h" => {
                        upper_limit = next_guess;
                    }
                    _ => {
                        lower_limit = next_guess;
                    }
                }
            }
        }
    }

    Ok(())
}
