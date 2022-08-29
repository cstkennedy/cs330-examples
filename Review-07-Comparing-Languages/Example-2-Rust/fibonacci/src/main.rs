use std::io::{Write, stdin, stdout};

/// Generate the Fibonacci Sequence to the n-th number.
/// 1 1 2 3 5 8 13 21 34...
/// <p>
/// The user must enter a number no smaller than 3 and
/// no greater than 20
fn main() {
    // Fibonaccci
    let mut fm2: u64 = 1;  // n-2 (previous previous) fibonacci number
    let mut fm1: u64 = 1;  // n-1 (previous) fibonacci number

    // Prompt the user
    print!("Generate how many numbers? ");
    let _ = stdout().flush();

    let mut index = String::new();
    stdin().read_line(&mut index)
        .expect("ERROR: No index was entered.");

    let index: u64 = index.trim().parse()
        .expect("ERROR: Not a legal integer.");

    // Print a blank line
    println!();

    // Note what happens if we do not check the
    // index entered by the user

    // De-Morgan's Law
    // !(index >= 3 && index <= 20)
    // !(index >= 3) || !(index <= 20)
    // (index < 3 || index > 20)
    if index < 3 || index > 20 {
        // Error Message
        print!("{:3} is not between 3 and 20\n", index);

        // Exit with an error state
        std::process::exit(1);
    }

    // Initial output
    println!("{:>2}: {:10}", 1, fm2);
    println!("{:>2}: {:10}", 2, fm1);

    // The first 2 numbers were already output
    for i in 3..(index + 1) {
        let f   = fm1 + fm2;
        fm2 = fm1;
        fm1 = f;

        println!("{:>2}: {:10}", i, f);
    }
}
