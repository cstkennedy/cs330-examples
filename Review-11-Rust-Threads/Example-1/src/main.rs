use std::env;

use std::thread;
use std::time::Instant;

use coin_flip::flip_task::FlipTask;

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
        .parse()
        .unwrap_or(DEFAULT_NUM_TRIALS);

    (num_threads, num_flips)
}

fn get_num_flips_per_thread(num_threads: usize, num_flips: u64) -> Vec<u64> {
    let num_flips_per_thread = num_flips / (num_threads as u64);
    let num_flips_truncated = num_flips - (num_flips_per_thread * (num_threads as u64));
    let num_flips_for_last = num_flips_per_thread + num_flips_truncated;

    let mut num_flips = vec![num_flips_per_thread; num_threads - 1];
    num_flips.push(num_flips_for_last);

    num_flips
}

fn run_parallel(num_threads: usize, num_flips: u64) {
    let num_flips = get_num_flips_per_thread(num_threads, num_flips);

    let handles: Vec<_> = num_flips
        .into_iter()
        .map(|num_flips_for_thread| {
            thread::spawn(move || {
                FlipTask::simulate_flips(num_flips_for_thread)
            })
        })
        .collect();

    let mut merged_results: FlipTask = FlipTask::default();
    for (idx, handle) in handles.into_iter().enumerate() {
        let result = handle.join().unwrap();

        println!("Worker {idx:>2} -> {}", result);
        merged_results += result;
    }
    println!("{}", "-".repeat(72));
    println!("Overall   -> {merged_results}");
}

fn main() {
    let (num_threads, num_flips) = __parse_args(env::args());

    if num_threads == 1 {
        let start = Instant::now();

        let result = FlipTask::simulate_flips(num_flips);
        println!("{}", result);

        let duration = start.elapsed();
        println!("{}", "-".repeat(72));
        println!("Sequential Time: {duration:?}");
    } else {
        let start = Instant::now();

        run_parallel(num_threads, num_flips);

        let duration = start.elapsed();
        println!("Parallel Time: {duration:?}");
    }
}
