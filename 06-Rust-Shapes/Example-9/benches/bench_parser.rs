use std::io::BufReader;

use stringreader::StringReader;
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

    let str_reader = StringReader::new(raw_str);
    let str_reader = BufReader::new(str_reader);
    let some_shapes = Parser::read_shapes(black_box(str_reader));

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

    let str_reader = StringReader::new(raw_str);
    let str_reader = BufReader::new(str_reader);
    let some_shapes = Parser::read_shapes_with(black_box(str_reader));
}

fn main() {
    divan::main();
}
