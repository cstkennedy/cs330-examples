use std::io::{Write, stdin, stdout};

use eyre::WrapErr;
use itertools;

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
fn main() -> eyre::Result<()> {
    //--------------------------------------------------------------------------
    // Prompt for sequence length and validate
    //--------------------------------------------------------------------------
    let index = {
        print!("Generate how many numbers? ");
        let _ = stdout().flush();

        let mut index = String::new();
        stdin()
            .read_line(&mut index)
            .wrap_err("ERROR: No index was entered.")?;

        index
            .trim()
            .parse::<u8>()
            .wrap_err("ERROR: Not a legal integer.")?
    };

    println!();

    if __validate_args(index) {
        eyre::bail!("{index:3} is not between 3 and 20");
    }

    //--------------------------------------------------------------------------
    // Compute and output the Fibonaccci Sequence to the index-th term
    //--------------------------------------------------------------------------
    let fibonacci_seq: Vec<u128> = itertools::iterate((0, 1), |(fm2, fm1)| {
        let f_next = fm1 + fm2;

        (*fm1, f_next)
    })
    .map(|(_prev, current)| current)
    .take(index.try_into()?)
    .collect();

    for (idx, fib_num) in (1..).zip(fibonacci_seq.iter()) {
        println!("{idx:>2}: {fib_num:10}");
    }
    Ok(())
}
