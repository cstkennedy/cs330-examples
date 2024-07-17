mod flip_task;

use std::thread;
use std::time::Instant;

use robusta_jni::bridge;
use robusta_jni::convert::Signature;

use flip_task::FlipTask;


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

#[bridge]
mod jni {
    use super::*;

    #[derive(Signature)]
    #[package()]
    pub struct Simulate;

    impl Simulate {
        pub extern "jni" fn doFlips(num_threads: i64, num_flips: i64) {

            let num_flips: u64 = num_flips.try_into().unwrap();
            if num_threads == 1 {
                let start = Instant::now();

                let result = FlipTask::simulate_flips(num_flips);
                println!("{}", result);

                let duration = start.elapsed();
                println!("{}", "-".repeat(72));
                println!("Sequential Time: {duration:?}");
            } else {
                let start = Instant::now();

                run_parallel(num_threads.try_into().unwrap(), num_flips);

                let duration = start.elapsed();
                println!("Parallel Time: {duration:?}");
            }
        }
    }
}
