use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;

use rand::prelude::*;
use rand::rngs::SmallRng;

use itertools::Itertools;

const KNOWN_SHAPES: [(&'static str, usize); 5] = [
    ("Triangle", 3),
    ("Right Triangle", 2),
    ("Equilateral Triangle", 1),
    ("Square", 1),
    ("Circle", 1),
];

const INVALID_SHAPE_RATE: f64 = 0.10;
const INVALID_DIM_RATE: f64 = 0.10;

pub fn random_file(num_lines: u64) -> Result<String, std::io::Error> {
    let random_shape_filename = format!("random-shapes-{num_lines}.txt");

    let random_shape_file = File::create(&random_shape_filename)?;
    let mut random_shape_file = BufWriter::new(random_shape_file);

    let mut rng: SmallRng = rand::make_rng();

    let mut num_invalid_shapes: u64 = 0;
    for _ in 0..num_lines {
        // Create an unknown shape"
        if rng.random::<f64>() < INVALID_SHAPE_RATE {
            num_invalid_shapes += 1;

            writeln!(random_shape_file, "LOL Not a Shape; 1 2 3 4 5")?;
            continue;
        }

        let (shape, dim_count) = KNOWN_SHAPES.choose(&mut rng).unwrap();
        let dims = (0..*dim_count)
            .map(|_| rng.random_range(0.0..10.0))
            .map(|dim| format!("{:.4}", dim))
            .join(" ");
        writeln!(random_shape_file, "{shape}; {dims}")?;
    }

    let _num_valid_shapes = num_lines - num_invalid_shapes;

    Ok(random_shape_filename)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let num_lines: u64 = args[1].parse().unwrap();

    random_file(num_lines).unwrap();
}
