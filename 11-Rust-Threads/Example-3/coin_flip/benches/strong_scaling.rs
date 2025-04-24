use divan::{black_box, Bencher};

use coin_flip::flip_task::FlipTask;
use coin_flip::run_parallel;

#[divan::bench(min_time = 1, args = [1, 2, 4, 8])]
fn bench_10_000(num_threads: usize) {
    let num_flips = 10_000;

    if num_threads == 1 {
        let _ = FlipTask::simulate_flips(black_box(num_flips));
    } else {
        let _ = run_parallel(black_box(num_threads), black_box(num_flips));
    }
}

#[divan::bench(min_time = 1, args = [1, 2, 4, 8])]
fn bench_1_000_000(num_threads: usize) {
    let num_flips = 1_000_000;

    if num_threads == 1 {
        let _ = FlipTask::simulate_flips(black_box(num_flips));
    } else {
        let _ = run_parallel(black_box(num_threads), black_box(num_flips));
    }
}

#[divan::bench(min_time = 1, args = [1, 2, 4, 8])]
fn bench_10_000_000(num_threads: usize) {
    let num_flips = 10_000_000;

    if num_threads == 1 {
        let _ = FlipTask::simulate_flips(black_box(num_flips));
    } else {
        let _ = run_parallel(black_box(num_threads), black_box(num_flips));
    }
}

fn main() {
    divan::main();
}
