use rand::prelude::*;
use std::env;
use std::thread;

use num_format::{Locale, ToFormattedString};

fn main() {
    let args: Vec<String> = env::args().collect();

    let num_threads: u64 = args[1].parse().unwrap_or(1);
    let num_flips: u64 = args[2].parse().unwrap();

    let mut handles = Vec::new();

    for _ in 0..num_threads {
        let num_flips_for_thread = num_flips / num_threads;
        handles.push(thread::spawn(move || {
            let mut rng = rand::thread_rng();

            let num_heads = (0..num_flips_for_thread)
                .map(|_| rng.gen::<bool>())
                .filter(|t_or_f| t_or_f == &true)
                .count() as u64;

            let num_tails = num_flips_for_thread - num_heads;

            (num_heads, num_tails)
        }));
    }

    for handle in handles {
        let (num_heads, num_tails) = handle.join().unwrap();
        println!(
            "Heads: {} | Tails: {}",
            num_heads.to_formatted_string(&Locale::en),
            num_tails.to_formatted_string(&Locale::en)
        );
    }
}
