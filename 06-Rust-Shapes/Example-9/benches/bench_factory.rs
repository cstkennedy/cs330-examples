// use criterion::{black_box, criterion_group, criterion_main, Criterion};
use divan::{black_box, Bencher};

use shapes::prelude::Factory;

use shapes::circle::Circle;
use shapes::equilateral_triangle::EquilateralTriangle;
use shapes::right_triangle::RightTriangle;
use shapes::square::Square;
use shapes::triangle::Triangle;

// Criterion
/*
fn bench_is_known(c: &mut Criterion) {
    c.bench_function("is_known", |b| {
        b.iter(|| {
            let _ = Factory::is_known(black_box("Circle"));
            let _ = Factory::is_known(black_box("Square"));
            let _ = Factory::is_known(black_box("Triangle"));
            let _ = Factory::is_known(black_box("Right Triangle"));
            let _ = Factory::is_known(black_box("Equilateral Triangle"));
        })
    });
}

fn bench_number_known(c: &mut Criterion) {
    c.bench_function("number_known", |b| {
        b.iter(|| {
            let _ = Factory::number_known();
        })
    });
}

fn bench_list_known(c: &mut Criterion) {
    c.bench_function("list_known", |b| {
        b.iter(|| {
            let _ = Factory::list_known();
        })
    });
}

fn bench_create(c: &mut Criterion) {
    // I need to write this test...
}

fn bench_create_with(c: &mut Criterion) {
    c.bench_function("create_with - all shapes", |b| {
        b.iter(|| {
            let _ = Factory::create_with(black_box("Triangle"), black_box(&[3.0, 4.0, 5.0]));
            let _ = Factory::create_with(black_box("Right Triangle"), black_box(&[3.0, 4.0]));
            let _ = Factory::create_with(black_box("Equilateral Triangle"), black_box(&[5.0]));
            let _ = Factory::create_with(black_box("Circle"), black_box(&[5.0]));
            let _ = Factory::create_with(black_box("Square"), black_box(&[5.0]));
        })
    });
}

criterion_group!(benches, bench_is_known, bench_number_known, bench_list_known, bench_create_with);
criterion_main!(benches);
*/

// Divan
const SHAPE_TUPLES: &'static [(&str, &[f64])] = &[
    (&"Triangle", &[3.0, 4.0, 5.0]),
    (&"Right Triangle", &[3.0, 4.0]),
    (&"Equilateral Triangle", &[5.0]),
    (&"Circle", &[5.0]),
    (&"Square", &[5.0]),
];

#[divan::bench(min_time = 1, args = ["Circle", "Square", "Triangle", "Right Triangle", "Equilateral Triangle"])]
fn bench_is_known_valid(name: &str) {
    let _ = Factory::is_known(black_box(name));
}

#[divan::bench(min_time = 1, args = ["Not Known"])]
fn bench_is_known_invalid(name: &str) {
    let _ = Factory::is_known(black_box(name));
}

#[divan::bench(min_time = 1)]
fn bench_number_known() {
    let _ = Factory::number_known();
}

#[divan::bench(min_time = 1)]
fn bench_list_known() {
    let _ = Factory::list_known();
}

#[divan::bench(min_time = 1, args = SHAPE_TUPLES)]
fn bench_create(name_and_vals: (&str, &[f64])) {
    let (name, _) = name_and_vals;

    let _ = Factory::create(black_box(&name));
}

#[divan::bench(min_time = 1, args = SHAPE_TUPLES)]
fn bench_create_with(name_and_vals: (&str, &[f64])) {
    let (name, dims) = name_and_vals;
    let _ = Factory::create_with(black_box(&name), black_box(&dims));
}

#[divan::bench(min_time = 1)]
fn bench_create_with_invalid() {
    let _ = Factory::create_with(black_box("Triangle"), black_box(&[1.0, 3.0]));
}

#[divan::bench(min_time = 1)]
fn bench_from_slice_circle() {
    let dims: &[f64] = &[5.0];
    let _ = Circle::from(black_box(dims));
}

#[divan::bench(min_time = 1)]
fn bench_from_slice_triangle() {
    let dims: &[f64] = &[4.0, 5.0, 6.0];
    let _ = Triangle::from(black_box(dims));
}

fn main() {
    divan::main();
}
