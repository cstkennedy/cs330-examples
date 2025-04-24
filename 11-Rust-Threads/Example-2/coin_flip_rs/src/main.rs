use std::env;
use std::time::{Duration, Instant};

use ::coin_flip::*;

const DEFAULT_NUM_TRIALS: u64 = 10_000;

fn __parse_args<I>(args: I) -> (usize, u64)
where
    I: Iterator<Item = String>,
{
    let args: Vec<String> = args.collect();

    /*
    let num_threads: usize = args[1].parse().unwrap_or(1);
    let num_threads = {
        if num_threads < 1 {
            1
        } else if num_threads > 32 {
            32
        } else {
            num_threads
        }
    };
    */
    let num_threads: usize = args[1].parse().unwrap_or(1);
    let num_threads = num_threads.clamp(1, 32);

    let num_flips: u64 = args[2]
        .replace(",", "")
        .replace("_", "")
        .parse()
        .unwrap_or(DEFAULT_NUM_TRIALS);

    (num_threads, num_flips)
}

fn main() {
    let (num_threads, num_flips) = __parse_args(env::args());

    run_parallel(num_threads, num_flips)
}
