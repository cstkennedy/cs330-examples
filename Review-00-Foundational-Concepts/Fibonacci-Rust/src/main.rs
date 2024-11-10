use std::io::{stdin, stdout, Write};

/// Note what happens if we do not check the
/// index entered by the user
///
/// De-Morgan's Law
/// !(index >= 3 && index <= 20)
/// !(index >= 3) || !(index <= 20)
/// (index < 3 || index > 20)
///
/// Rust
/// (3..=20).contains(...)
fn __validate_args(index: u8) -> bool {
    !(3..=20).contains(&index)
}

/// Generate the Fibonacci Sequence to the n-th number.
/// 1 1 2 3 5 8 13 21 34...
/// <p>
/// The user must enter a number no smaller than 3 and
/// no greater than 20
fn main() {
    //--------------------------------------------------------------------------
    // Prompt for sequence length and validate
    //--------------------------------------------------------------------------
    print!("Generate how many numbers? ");
    let _ = stdout().flush();

    let index = {
        let mut index = String::new();
        stdin()
            .read_line(&mut index)
            .expect("ERROR: No index was entered.");

        index.trim().parse::<u8>()
            .expect("ERROR: Not a legal integer.")
    };

    println!();

    if __validate_args(index) {
        println!("{index:3} is not between 3 and 20");
        std::process::exit(1);
    }

    //--------------------------------------------------------------------------
    // Compute and output the Fibonaccci Sequence to the index-th term
    //--------------------------------------------------------------------------
    let mut fm2: u64 = 1; // n-2 (previous previous) fibonacci number
    let mut fm1: u64 = 1; // n-1 (previous) fibonacci number

    println!("{:>2}: {fm2:10}", 1);
    println!("{:>2}: {fm1:10}", 2);

    // The first 2 numbers were already output
    for i in 3..=index {
        let f = fm1 + fm2;
        fm2 = fm1;
        fm1 = f;

        println!("{i:>2}: {f:10}");
    }
}
