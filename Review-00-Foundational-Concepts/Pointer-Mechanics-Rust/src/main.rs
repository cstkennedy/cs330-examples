use std::cell::LazyCell;

const D_LINE: LazyCell<String> = LazyCell::new(|| "-".repeat(48));

const NUM_POWERS: usize = 32;

fn main() {
    let powers_of_two = {
        let mut powers_of_two = [0i32; NUM_POWERS];

        for i in 0..powers_of_two.len() {
            powers_of_two[i] = 1 << i;
        }
        powers_of_two
    };

    println!("{}", *D_LINE);
    for val in powers_of_two.iter() {
        println!("{}", val);
    }

    println!();
    println!("{}", *D_LINE);
    println!();

    for i in 0..powers_of_two.len() {
        println!(
            "{:>2}: {:>11} {:>32b}",
            i, powers_of_two[i], powers_of_two[i]
        );
    }

    println!();
    println!("{}", *D_LINE);
    println!();

    for i in 0..powers_of_two.len() {
        println!(
            "{:>2}: {:>11} {:>32p}",
            i, powers_of_two[i], &powers_of_two[i]
        );
    }
    println!();

    let size_in_bytes = std::mem::size_of::<i32>();
    let size_in_bits = size_in_bytes << 3;

    println!(
        "size_of(i32):       {:>4} bytes / {:>4} bits",
        size_in_bytes, size_in_bits
    );

    let size_in_bytes = std::mem::size_of_val(&powers_of_two);
    let size_in_bits = size_in_bytes << 3;

    println!(
        "size_of([i32; 32]): {:>4} bytes / {:>4} bits",
        size_in_bytes, size_in_bits
    );
}
