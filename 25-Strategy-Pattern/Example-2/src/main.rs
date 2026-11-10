use number_guess::{
    guessers::BisectionGuesser, selectors::HumanSelector,
};

fn main() -> eyre::Result<()> {
    game_loop(
        BisectionGuesser::from(0..=100_u32),
        HumanSelector::default(),
    )
}

pub fn game_loop<G, P>(mut guesser: G, mut selector: P) -> eyre::Result<()>
where
    G: number_guess::traits::NumberGuesser,
    P: number_guess::traits::NumberSelector,
{
    loop {
        let guess = guesser.ask_if_number();
        match selector.answer_yes_or_no(guess)?.as_ref() {
            "y" => {
                guesser.announce_win();
                break;
            }
            _ => {
                guesser.ask_if_too_high_or_low(guess);
                let response = selector.answer_too_high_or_low(guess)?;

                guesser.update_range(response.as_ref());
            }
        }
    }

    Ok(())
}
