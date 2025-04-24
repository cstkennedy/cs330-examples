use num_format::{Locale, ToFormattedString};
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::ops::Add;
use std::ops::AddAssign;

#[derive(Default)]
pub struct FlipTask {
    pub num_flips: u64,
    pub num_heads: u64,
    pub num_tails: u64,
}

impl FlipTask {
    pub fn simulate_flips(num_flips: u64) -> FlipTask {
        let mut rng = SmallRng::from_rng(&mut rand::rng());

        let num_heads = (0..num_flips)
            .map(|_| rng.random::<bool>())
            .filter(|val| val == &true)
            .count() as u64;

        let num_tails = num_flips - num_heads;

        FlipTask {
            num_flips,
            num_heads,
            num_tails,
        }
    }

    fn percent_heads(&self) -> f64 {
        self.num_heads as f64 / self.num_flips as f64
    }

    fn percent_tails(&self) -> f64 {
        self.num_tails as f64 / self.num_flips as f64
    }
}

impl Add for FlipTask {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            num_flips: self.num_flips + other.num_flips,
            num_heads: self.num_heads + other.num_heads,
            num_tails: self.num_tails + other.num_tails,
        }
    }
}

impl AddAssign for FlipTask {
    fn add_assign(&mut self, other: Self) {
        self.num_flips += other.num_flips;
        self.num_heads += other.num_heads;
        self.num_tails += other.num_tails;
    }
}

impl std::fmt::Display for FlipTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "# Heads: {:>6} ({:>6.4}) / # Tails {:>6} ({:>6.4})",
            self.num_heads.to_formatted_string(&Locale::en),
            self.percent_heads(),
            self.num_tails.to_formatted_string(&Locale::en),
            self.percent_tails()
        )
    }
}
