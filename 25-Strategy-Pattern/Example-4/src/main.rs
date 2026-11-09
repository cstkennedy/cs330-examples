use number_guess::{
    guessers::{BisectionGuesser, LowGuesser, HighGuesser},
    selectors::HumanSelector,
    traits::ResponseYesOrNo,
};

fn main() -> eyre::Result<()> {
    game_loop(
        BisectionGuesser::from(0..=100_u32),
        HumanSelector::default(),
    )?;

    println!("{}", "*".repeat(40));
    game_loop(LowGuesser::from(0..=100_u32), HumanSelector::default())?;

    println!("{}", "*".repeat(40));
    game_loop(HighGuesser::from(0..=100_u32), HumanSelector::default())
}

pub fn game_loop<G, P>(mut guesser: G, selector: P) -> eyre::Result<()>
where
    G: number_guess::traits::NumberGuesser,
    P: number_guess::traits::NumberSelector,
{
    loop {
        if let ResponseYesOrNo::Yes = guesser.ask_if_number(&selector) {
            break;
        }
    }

    Ok(())
}
