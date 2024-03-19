use rand::prelude::*;
use std::thread;

fn main() {
    let mut handles = Vec::new();

    let num_threads = 4;

    for _ in 0..num_threads {
        let num_flips = 1_000_000_000;
        handles.push(thread::spawn(move || {
            let mut rng = rand::thread_rng();

            let num_heads = (0..num_flips)
                .map(|_| rng.gen::<bool>())
                .filter(|t_or_f| t_or_f == &true)
                .count();

            let num_tails = num_flips - num_heads;

            (num_heads, num_tails)
        }));
    }

    for handle in handles {
        let (num_heads, num_tails) = handle.join().unwrap();
        println!("Heads: {} | Tails: {}", num_heads, num_tails);
    }
}
