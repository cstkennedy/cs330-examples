use std::env;
use std::time::{Duration, Instant};

use ::coin_flip::flip_task::FlipTask;
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
    let start = Instant::now();
    let (overall, results) = if num_threads == 1 {
        let result = FlipTask::simulate_flips(num_flips);

        (result, vec![result])
    } else {
        run_parallel(num_threads, num_flips)
    };
    let duration = start.elapsed();

    let summary = results
        .iter()
        .enumerate()
        .map(|(idx, result)| -> String { format!("Worker {idx:>2} -> {:}", result) })
        .collect::<Vec<String>>()
        .join("\n");

    println!(
        "{}\n{}\n{}\n{}",
        summary,
        "-".repeat(72),
        format_args!("Overall   -> {}", overall),
        format_args!(
            "{}: {:?}",
            match num_threads {
                1 => "Sequential Time",
                _ => "Parallel Time",
            },
            duration
        )
    );
}
