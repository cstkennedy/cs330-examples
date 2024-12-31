use std::env;

/// Output all even numbers in a given range.
///
/// # Arguments
///
///   - lower lower integer bound (a)
///   - upper upper integer bound (b)
///
fn output_even_integers(lower: i64, upper: i64) {
    let first_even = if lower % 2 == 0 {
        lower
    }
    else {
        lower + 1
    };

    for next_even in (first_even..=upper).step_by(2) {
        println!("{next_even}");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    // Check and parse command line args
    if argc < 3 {
       println!(" Usage: ./even_gen [lower_bound] [upper_bound]");
       std::process::exit(1);
    }

    // Assume all args are well formed (i.e., can be parsed as integers).
    let lower_bound: i64 = args[1].parse().unwrap();
    let upper_bound: i64 = args[2].parse().unwrap();

    // The core even output logic
    println!("Range [{lower_bound}, {upper_bound}]");
    println!();

    output_even_integers(lower_bound, upper_bound);
}
