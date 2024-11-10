use std::fmt::Display;

fn print_multiple_times<T>(value: T, count: u64)
where
    T: Display,
{
    for _i in 0..(count - 1) {
        print!("{} ", value);
    }
    println!("{}", value);
}

fn main() {
    print_multiple_times(7, 3);
    println!();
    print_multiple_times(5.5_f64, 2);
    println!();
    print_multiple_times(5.5_f32, 2);
    println!();
    print_multiple_times("Hello", 4);
    println!();
    print_multiple_times('?', 20);
}
