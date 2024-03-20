use rand::prelude::*;
use rand::rngs::SmallRng;
use std::ops::Add;
use std::ops::AddAssign;
use num_format::{Locale, ToFormattedString};

pub struct FlipTask {
    pub num_flips: u64,
    pub num_heads: u64,
    pub num_tails: u64,
}


impl FlipTask {
    pub fn simulate_flips(num_flips: u64) -> FlipTask {
        let mut rng = SmallRng::from_thread_rng();

        let num_heads = (0..num_flips)
            .into_iter()
            .map(|_| rng.gen::<bool>())
            .filter(|val| val == &true)
            .count() as u64;

        let num_tails = num_flips - num_heads;

        FlipTask {
            num_flips,
            num_heads,
            num_tails,
        }
    }
}

impl Default for FlipTask {
    fn default() -> Self {
        FlipTask {
            num_flips: 0,
            num_heads: 0,
            num_tails: 0,
        }
    }
}

impl Add for FlipTask {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            num_flips: self.num_flips + other.num_flips,
            num_heads: self.num_heads + other.num_flips,
            num_tails: self.num_tails + other.num_tails,
        }
    }
}

impl AddAssign for FlipTask {

    fn add_assign(&mut self, other: Self) {
        self.num_flips += other.num_flips;
        self.num_heads += other.num_flips;
        self.num_tails += other.num_tails;
    }
}


impl std::fmt::Display for FlipTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "# Heads: {:>6} ({:>6.4}) / # Tails {:>6} ({:>6.4})",
            self.num_heads.to_formatted_string(&Locale::en),
            0,
            self.num_tails.to_formatted_string(&Locale::en),
            0
        )
    }
}

