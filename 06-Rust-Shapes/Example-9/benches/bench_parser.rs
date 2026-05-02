use std::io::BufReader;

use divan::{black_box, Bencher};

use shapes::prelude::Parser;

#[divan::bench(min_time = 1)]
fn bench_read_shapes() {
    // The read function should handle (i.e., ignore) leading whitespace
    let raw_str = r#"
        Triangle
        Right Triangle
        Equilateral Triangle
        Square
        Circle
        1337 Haxor"#;

    let some_shapes = Parser::read_shapes(black_box(raw_str.as_bytes()));

}

#[divan::bench(min_time = 1)]
fn test_bench_shapes_with() {
    // The read function should handle (i.e., ignore) leading whitespace
    let raw_str = r#"
        Triangle; 1 2 3
        Right Triangle; 3 4
        Equilateral Triangle; 5
        Square; 5
        Circle; 5
        1337 Haxor; invalid input"#;

    let some_shapes = Parser::read_shapes(black_box(raw_str.as_bytes()));
}

fn main() {
    divan::main();
}
