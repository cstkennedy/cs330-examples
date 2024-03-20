use rand::prelude::*;
use rand::rngs::SmallRng;
use std::env;
use std::thread;
use std::time::{Instant};

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

fn simulate_flips(num_flips: u64) -> (u64, u64) {
    let mut rng = SmallRng::from_thread_rng();

    let num_heads = (0..num_flips)
        .into_iter()
        .map(|_| rng.gen::<bool>())
        .filter(|val| val == &true)
        .count() as u64;

    let num_tails = num_flips - num_heads;

    (num_heads, num_tails)
}

fn main() {
    let (num_threads, num_flips) = __parse_args(env::args());

    let start = Instant::now();

    if num_threads == 1 {
        let (num_heads, num_tails) = simulate_flips(num_flips);
        println!(
            "Heads: {} | Tails: {}",
            num_heads.to_formatted_string(&Locale::en),
            num_tails.to_formatted_string(&Locale::en)
        );
    }
    else {
        let num_flips_per_thread = num_flips / (num_threads as u64);
        let num_flips_truncated = num_flips - (num_flips_per_thread * (num_threads as u64));
        let num_flips_for_last = num_flips_per_thread + num_flips_truncated;

        let mut num_flips = vec![num_flips_per_thread; num_threads - 1];
        num_flips.push(num_flips_for_last);

        let handles: Vec<_> = num_flips
            .into_iter()
            .map(|num_flips_for_thread| thread::spawn(move || simulate_flips(num_flips_for_thread)))
            .collect();

        for handle in handles {
            let (num_heads, num_tails) = handle.join().unwrap();
            println!(
                "Heads: {} | Tails: {}",
                num_heads.to_formatted_string(&Locale::en),
                num_tails.to_formatted_string(&Locale::en)
            );
        }
    }
    let duration = start.elapsed();

    println!("Seconds: {:?}", duration);
}
