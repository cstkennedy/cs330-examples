// use criterion::{black_box, criterion_group, criterion_main, Criterion};
use divan::{Bencher, black_box};

use shapes::prelude::Factory;

use shapes::circle::Circle;
use shapes::equilateral_triangle::EquilateralTriangle;
use shapes::right_triangle::RightTriangle;
use shapes::square::Square;
use shapes::triangle::Triangle;

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
fn bench_from_slice_square() {
    let dims: &[f64] = &[5.0];
    let _ = Square::from(black_box(dims));
}

#[divan::bench(min_time = 1)]
fn bench_from_slice_triangle() {
    let dims: &[f64] = &[4.0, 5.0, 6.0];
    let _ = Triangle::from(black_box(dims));
}

#[divan::bench(min_time = 1)]
fn bench_from_slice_right_triangle() {
    let dims: &[f64] = &[4.0, 5.0];
    let _ = RightTriangle::from(black_box(dims));
}

#[divan::bench(min_time = 1)]
fn bench_from_slice_equilateral_triangle() {
    let dims: &[f64] = &[4.0];
    let _ = EquilateralTriangle::from(black_box(dims));
}

fn main() {
    divan::main();
}
