use std::io::BufReader;
use std::str::FromStr;

use divan::{Bencher, black_box};

use shapes::factory::{CreationFactory, Factory, FactoryDirectory};
use shapes::monoshape::MonoFactory;
use shapes::prelude::Parser;

const LINES: [&str; 6] = [
    "Triangle; 1 2 3",
    "Right Triangle; 3 4",
    "Equilateral Triangle; 5",
    "Square; 5",
    "Circle; 5",
    "1337 Haxor; invalid input",
];

#[divan::bench(min_time = 1, args = LINES, types = [Factory, MonoFactory])]
fn test_bench_parse_shape_line<F>(line: &str)
where
    F: CreationFactory,
    F::Item: FromStr,
{
    let _ = F::Item::from_str(black_box(line));
}

#[divan::bench(min_time = 1, types = [Factory, MonoFactory])]
fn bench_read_shapes<F>(bencher: Bencher)
where
    F: CreationFactory,
    F::Item: std::str::FromStr,
{
    // The read function should handle (i.e., ignore) leading whitespace
    let raw_str = r#"
        Triangle
        Right Triangle
        Equilateral Triangle
        Square
        Circle
        1337 Haxor"#;

    bencher
        .with_inputs(|| {
            let str_reader = BufReader::new(raw_str.as_bytes());

            str_reader
        })
        .bench_values(|str_reader| {
            let some_shapes = Parser::<F>::read_shapes(black_box(str_reader));
        });
}

#[divan::bench(min_time = 1, types = [Factory, MonoFactory])]
fn test_bench_shapes_with<F>(bencher: Bencher)
where
    F: CreationFactory,
    F::Item: std::str::FromStr,
{
    // The read function should handle (i.e., ignore) leading whitespace
    let raw_str = r#"
        Triangle; 1 2 3
        Right Triangle; 3 4
        Equilateral Triangle; 5
        Square; 5
        Circle; 5
        1337 Haxor; invalid input"#;

    bencher
        .with_inputs(|| {
            let str_reader = BufReader::new(raw_str.as_bytes());

            str_reader
        })
        .bench_values(|str_reader| {
            let some_shapes = Parser::<F>::read_shapes_with(black_box(str_reader));
        });
}

fn main() {
    divan::main();
}
