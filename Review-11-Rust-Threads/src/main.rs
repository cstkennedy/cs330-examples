use rand::prelude::*;
use rand::rngs::SmallRng;
use std::env;
use std::ops::Add;
use std::ops::AddAssign;
use std::thread;
use std::time::Instant;

use coin_flip::flip_task::FlipTask;


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
