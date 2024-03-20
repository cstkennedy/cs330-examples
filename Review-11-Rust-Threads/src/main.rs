use rand::prelude::*;
use rand::rngs::SmallRng;
use std::env;
use std::ops::Add;
use std::ops::AddAssign;
use std::thread;
use std::time::Instant;

use num_format::{Locale, ToFormattedString};

fn __parse_args<I>(args: I) -> (usize, u64)
where
    I: Iterator<Item = String>,
{
    let args: Vec<String> = args.collect();

    let num_threads: usize = args[1].parse().unwrap_or(1);
    let num_threads = {
        if num_threads < 1 {
            1
        } else {
            num_threads
        }
    };

    let num_flips: u64 = args[2].replace(",", "").parse().unwrap();

    (num_threads, num_flips)
}

struct FlipTask {
    num_flips: u64,
    num_heads: u64,
    num_tails: u64,
}


impl FlipTask {
    fn simulate_flips(num_flips: u64) -> FlipTask {
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

fn main() {
    let (num_threads, num_flips) = __parse_args(env::args());

    let start = Instant::now();

    if num_threads == 1 {
        let result = FlipTask::simulate_flips(num_flips);
        println!("{}", result);
    } else {
        let num_flips_per_thread = num_flips / (num_threads as u64);
        let num_flips_truncated = num_flips - (num_flips_per_thread * (num_threads as u64));
        let num_flips_for_last = num_flips_per_thread + num_flips_truncated;

        let mut num_flips = vec![num_flips_per_thread; num_threads - 1];
        num_flips.push(num_flips_for_last);

        let handles: Vec<_> = num_flips
            .into_iter()
            .map(|num_flips_for_thread| {
                thread::spawn(move || FlipTask::simulate_flips(num_flips_for_thread))
            })
            .collect();

        let mut merged_results: FlipTask = Default::default();
        for (idx, handle) in handles.into_iter().enumerate() {
            let result = handle.join().unwrap();
            println!("Worker {idx:>2} -> {}", result);

            merged_results += result;
        }
        println!("Overall   -> {merged_results}");
    }
    let duration = start.elapsed();


    // println!(f"Total Trials -> {merged_results.total_flips:,}")

    println!("Seconds: {:?}", duration);
}
