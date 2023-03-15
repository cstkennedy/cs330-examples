fn main() {
    let D_LINE = std::iter::repeat("-").take(48).collect::<String>();

    const num_powers: usize = 32;

    let mut powers_of_two = [0i32; num_powers];

    println!("{}", D_LINE);

    for i in 0..powers_of_two.len() {
        powers_of_two[i] = (1 << i);
    }

    for val in powers_of_two.iter() {
        println!("{}", val);
    }

    println!();
    println!("{}", D_LINE);
    println!();

    for i in 0..powers_of_two.len() {
        println!("{:>2}: {:>11} {:>32b}", i, powers_of_two[i], powers_of_two[i]);
    }

    println!();
    println!("{}", D_LINE);
    println!();

    for i in 0..powers_of_two.len() {
        println!("{:>2}: {:>11} {:>32p}", i, powers_of_two[i], &powers_of_two[i]);
    }

    let size_in_bytes = std::mem::size_of::<u32>();
    let size_in_bits = size_in_bytes << 3;

    println!();
    println!("size_of(u32): {} bytes / {} bits", size_in_bytes, size_in_bits);

    let size_in_bytes = std::mem::size_of_val(&powers_of_two);
    let size_in_bits = size_in_bytes << 3;

    println!();
    println!("size_of([u32; 32]): {} bytes / {} bits", size_in_bytes, size_in_bits);
}
