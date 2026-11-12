use divan::{Bencher, black_box};

use shapes::error::CreationError;

const SHAPE_NAMES: &'static [&str] = &[
    &"Triangle",
    &"Right Triangle",
    &"Equilateral Triangle",
    &"Circle",
    &"Square",
];

#[divan::bench(min_time = 1, args = SHAPE_NAMES)]
fn bench_create_error_unknown_shape(name: &str) {
    let _ = CreationError::UnknownShapeError(black_box(name.into()));
}

#[divan::bench(min_time = 1, args = SHAPE_NAMES)]
fn bench_create_error_dimension_count(name: &'static str) {
    let _ = CreationError::DimensionCountError {
        name: black_box(name),
        num_required: black_box(5),
    };
}

#[divan::bench(min_time = 1, args = SHAPE_NAMES)]
fn bench_create_error_malformed_line(name: &str) {
    let _ = CreationError::MalformedLineError(black_box(name.into()));
}

fn main() {
    divan::main();
}
