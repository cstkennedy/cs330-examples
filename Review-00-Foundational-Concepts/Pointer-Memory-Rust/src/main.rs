use std::mem::size_of;
use std::ptr::addr_of;

fn main() {
    let x: i32 = 9001_i32;
    let x_pointer: *const i32 = &x;

    println!(
        "{:^12} | {:^18} | {:^16} | {:^16}",
        "Type", "Variable", "Address", "Value"
    );

    println!(
        "{:>12} | {:>18} | {:16p} | {:16}",
        "i32",
        "x",
        addr_of!(x),
        x
    );

    println!(
        "{:>12} | {:>18} | {:16p} | {:16p}",
        "*const i32",
        "x_pointer",
        addr_of!(x_pointer),
        x_pointer
    );

    println!("{}", "-".repeat(71));

    let x_box_box: Box<i32> = Box::new(9001);
    println!(
        "{:>12} | {:>18} | {:16p} | {:16p}",
        "Box<i32>",
        "x_box_box",
        addr_of!(x_box_box),
        x_box_box
    );
    println!();

    let size_in_bytes = size_of::<i32>();
    let size_in_bits = size_in_bytes << 3;
    println!(
        "{:<21}: {size_in_bytes:>4} bytes / {size_in_bits:>4} bits",
        "size_of::<i32>"
    );

    let size_in_bytes = size_of::<*const i32>();
    let size_in_bits = size_in_bytes << 3;
    println!(
        "{:<21}: {size_in_bytes:>4} bytes / {size_in_bits:>4} bits",
        "size_of::<*const i32>"
    );

    let size_in_bytes = size_of::<Box<i32>>();
    let size_in_bits = size_in_bytes << 3;
    println!(
        "{:<21}: {size_in_bytes:>4} bytes / {size_in_bits:>4} bits",
        "size_of::<Box<i32>>"
    );

    // Reference: <https://doc.rust-lang.org/std/mem/fn.size_of.html>
    // Box<T>, *const T, and Option<&T> all have the same size. If T is sized... then the size is
    // usize
}
