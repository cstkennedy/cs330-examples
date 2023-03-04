/// Maximum allowed array size
const MAX_SIZE: usize = 10;

/// 20 Dash Divider
const DIVIDER: &'static str = "--------------------";

fn main() {
    static_array_demo();
}

///
/// Statically allocated array demo - keep track of what is actually used
///
fn static_array_demo() {
    let mut names: [&str; MAX_SIZE] = [""; MAX_SIZE];
    let mut used: usize = 0;

    names[used] = "Thomas";
    used += 1;
    names[used] = "Jay";
    used += 1;

    println!("{}", DIVIDER);
    for i in 0..used {
        println!("{}", names[i]);
    }

    println!("{}", DIVIDER);
    print_array(&names);
}

///Print an array using the usual index based for loop.
///
/// @param toPrint the array to print
/// @param length size of the array (number of elements)
///
fn print_array(to_print: &[&str]) {
    for value in to_print.iter() {
        println!("{}", value);
    }
}
