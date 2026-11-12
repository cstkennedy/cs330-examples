#[cfg(test)]
use hamcrest2::prelude::*;

use shapes::factory::{CreationFactory, FactoryDirectory};
use shapes::prelude::Factory;

use shapes::monoshape::MonoFactory;

use shapes::circle::Circle;
use shapes::equilateral_triangle::EquilateralTriangle;
use shapes::right_triangle::RightTriangle;
use shapes::square::Square;
use shapes::triangle::Triangle;

use rstest::rstest;

#[rstest]
fn test_create_default_success() {
    // I need to write this test...
}

#[rstest]
#[case::Factory(Factory)]
#[case::MonoFactory(MonoFactory)]
fn test_create_default_failure<F>(#[case] _case: F)
where
    F: CreationFactory,
    F::Item: std::fmt::Debug + std::fmt::Display,
    F::Error: std::fmt::Debug,
{
    assert_that!(F::create_default("UnknownShape"), is(err()));
}

#[rstest]
#[case::Factory(Factory)]
#[case::MonoFactory(MonoFactory)]
fn test_create_with_circle<F>(#[case] _case: F)
where
    F: CreationFactory,
    F::Item: std::fmt::Debug + std::fmt::Display,
    F::Error: std::fmt::Debug,
{
    let a_shape = F::create_with("Circle", &[5.0]).unwrap();
    let ref_shape = Circle::new(5.0);
    assert_that!(a_shape.to_string(), equal_to(ref_shape.to_string()));
}

#[rstest]
#[case::Factory(Factory)]
#[case::MonoFactory(MonoFactory)]
fn test_create_with_square<F>(#[case] _case: F)
where
    F: CreationFactory,
    F::Item: std::fmt::Debug + std::fmt::Display,
    F::Error: std::fmt::Debug,
{
    let a_shape = F::create_with("Square", &[5.0]).unwrap();
    let ref_shape = Square::new(5.0);
    assert_that!(a_shape.to_string(), equal_to(ref_shape.to_string()));
}

#[rstest]
fn test_create_with_triangle() {
    let a_shape = Factory::create_with("Triangle", &[3.0, 4.0, 5.0]).unwrap();
    let ref_shape = Triangle::new(3.0, 4.0, 5.0);
    assert_that!(a_shape.to_string(), equal_to(ref_shape.to_string()));
}

#[rstest]
fn test_create_with_right_triangle() {
    let a_shape = Factory::create_with("Right Triangle", &[3.0, 4.0]).unwrap();
    let ref_shape = RightTriangle::new(3.0, 4.0);
    assert_that!(a_shape.to_string(), equal_to(ref_shape.to_string()));
}

#[rstest]
fn test_create_with_equilateral_triangle() {
    let a_shape = Factory::create_with("Equilateral Triangle", &[5.0]).unwrap();
    let ref_shape = EquilateralTriangle::new(5.0);
    assert_that!(a_shape.to_string(), equal_to(ref_shape.to_string()));
}

#[rstest]
#[case::Circle("Circle")]
#[case::Square("Square")]
#[case::Triangle("Triangle")]
#[case::RightTriangle("Right Triangle")]
#[case::EquilateralTriangle("Equilateral Triangle")]
fn test_is_known<F>(#[case] shape_name: &str, #[values(Factory, MonoFactory)] _case: F)
where
    F: FactoryDirectory,
{
    assert!(F::is_known(shape_name));
}

#[rstest]
#[case::Factory(Factory)]
#[case::MonoFactory(MonoFactory)]
fn test_list_known_contains<F>(#[case] _case: F)
where
    F: FactoryDirectory,
{
    let f_str = F::list_known();

    assert!(f_str.contains("  Circle"));
    assert!(f_str.contains("  Square"));
    assert!(f_str.contains("  Triangle"));
    assert!(f_str.contains("  Right Triangle"));
    assert!(f_str.contains("  Equilateral Triangle"));
}

#[rstest]
#[case::Factory(Factory)]
fn test_number_known<F>(#[case] _case: F)
where
    F: FactoryDirectory,
{
    assert_that!(F::number_known(), is(equal_to(5)));
}
