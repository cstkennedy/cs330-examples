use divan::{Bencher, black_box};

use shapes::factory::{CreationFactory, Factory, FactoryDirectory};
use shapes::monoshape::MonoFactory;

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

const SHAPE_NAMES: &'static [&str] = &[
    &"Triangle",
    &"Right Triangle",
    &"Equilateral Triangle",
    &"Circle",
    &"Square",
];

#[divan::bench(
    min_time = 1,
    args = ["Circle", "Square", "Triangle", "Right Triangle", "Equilateral Triangle"],
    types = [Factory, MonoFactory]
)]
fn bench_is_known_valid<F>(name: &str)
where
    F: FactoryDirectory,
{
    let _ = F::is_known(black_box(name));
}

#[divan::bench(min_time = 1, args = ["Not Known"], types = [Factory, MonoFactory])]
fn bench_is_known_invalid<F>(name: &str)
where
    F: FactoryDirectory,
{
    let _ = F::is_known(black_box(name));
}

#[divan::bench(min_time = 1, types = [Factory, MonoFactory])]
fn bench_number_known<F>()
where
    F: FactoryDirectory,
{
    let _ = F::number_known();
}

#[divan::bench(min_time = 1, types = [Factory, MonoFactory])]
fn bench_list_known<F>()
where
    F: FactoryDirectory,
{
    let _ = F::list_known();
}

#[divan::bench(min_time = 1, args = SHAPE_NAMES, types = [Factory, MonoFactory])]
fn bench_create<F>(name: &str)
where
    F: CreationFactory,
{
    let _ = F::create_default(black_box(&name));
}

#[divan::bench(min_time = 1, types = [Factory, MonoFactory])]
fn bench_create_invalid_name<F>()
where
    F: CreationFactory,
{
    let _ = F::create_default(black_box("Unknown Shape"));
}

#[divan::bench(min_time = 1, args = SHAPE_TUPLES, types = [Factory, MonoFactory])]
fn bench_create_with<F>(name_and_vals: (&str, &[f64]))
where
    F: CreationFactory,
{
    let (name, dims) = name_and_vals;
    let _ = Factory::create_with(black_box(&name), black_box(&dims));
}

#[divan::bench(min_time = 1, types = [Factory, MonoFactory])]
fn bench_create_with_invalid_name<F>()
where
    F: CreationFactory,
{
    let _ = F::create_with(black_box("Unknown Shape"), black_box(&[1.0, 3.0]));
}

#[divan::bench(min_time = 1, args = SHAPE_NAMES, types = [Factory, MonoFactory])]
fn bench_create_with_invalid_dims<F>(name: &str)
where
    F: CreationFactory,
{
    let _ = F::create_with(black_box(&name), black_box(&[1.0, 3.0, 5.0, 7.0, 8.0, 9.0]));
}

#[divan::bench(min_time = 1)]
fn bench_from_slice_circle_ok() {
    let dims: &[f64] = &[5.0];
    let _ = Circle::try_from(black_box(dims));
}

#[divan::bench(min_time = 1)]
fn bench_from_slice_circle_err() {
    let dims: &[f64] = &[5.0, 6.0];
    let _ = Circle::try_from(black_box(dims));
}

#[divan::bench(min_time = 1)]
fn bench_from_slice_square_ok() {
    let dims: &[f64] = &[5.0];
    let _ = Square::try_from(black_box(dims));
}

#[divan::bench(min_time = 1)]
fn bench_from_slice_square_err() {
    let dims: &[f64] = &[5.0, 7.0];
    let _ = Square::try_from(black_box(dims));
}

#[divan::bench(min_time = 1)]
fn bench_from_slice_triangle_ok() {
    let dims: &[f64] = &[4.0, 5.0, 6.0];
    let _ = Triangle::try_from(black_box(dims));
}

#[divan::bench(min_time = 1)]
fn bench_from_slice_triangle_err() {
    let dims: &[f64] = &[4.0, 5.0];
    let _ = Triangle::try_from(black_box(dims));
}

#[divan::bench(min_time = 1)]
fn bench_from_slice_right_triangle_ok() {
    let dims: &[f64] = &[4.0, 5.0];
    let _ = RightTriangle::try_from(black_box(dims));
}

#[divan::bench(min_time = 1)]
fn bench_from_slice_right_triangle_err() {
    let dims: &[f64] = &[4.0];
    let _ = RightTriangle::try_from(black_box(dims));
}

#[divan::bench(min_time = 1)]
fn bench_from_slice_equilateral_triangle_ok() {
    let dims: &[f64] = &[4.0];
    let _ = EquilateralTriangle::try_from(black_box(dims));
}

#[divan::bench(min_time = 1)]
fn bench_from_slice_equilateral_triangle_err() {
    let dims: &[f64] = &[4.0, 2.0];
    let _ = EquilateralTriangle::try_from(black_box(dims));
}

fn main() {
    divan::main();
}
