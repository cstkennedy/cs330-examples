fn main() {
    let d_line = "-".repeat(48);

    const NUM_POWERS: usize = 32;

    let mut powers_of_two = [0i32; NUM_POWERS];

    println!("{}", d_line);

    for i in 0..powers_of_two.len() {
        powers_of_two[i] = 1 << i;
    }

    for val in powers_of_two.iter() {
        println!("{}", val);
    }

    println!();
    println!("{}", d_line);
    println!();

    for i in 0..powers_of_two.len() {
        println!(
            "{:>2}: {:>11} {:>32b}",
            i, powers_of_two[i], powers_of_two[i]
        );
    }

    println!();
    println!("{}", d_line);
    println!();

    for i in 0..powers_of_two.len() {
        println!(
            "{:>2}: {:>11} {:>32p}",
            i, powers_of_two[i], &powers_of_two[i]
        );
    }

    let size_in_bytes = std::mem::size_of::<i32>();
    let size_in_bits = size_in_bytes << 3;

    println!();
    println!(
        "size_of(i32): {} bytes / {} bits",
        size_in_bytes, size_in_bits
    );

    let size_in_bytes = std::mem::size_of_val(&powers_of_two);
    let size_in_bits = size_in_bytes << 3;

    println!();
    println!(
        "size_of([i32; 32]): {} bytes / {} bits",
        size_in_bytes, size_in_bits
    );
}
