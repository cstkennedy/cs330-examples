use std::env;

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

    for next_even in (lower_bound..=upper_bound) {
        println!("{next_even}");
    }
}
