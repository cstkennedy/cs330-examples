#[cfg(test)]
use hamcrest2::prelude::*;

use shapes::prelude::Factory;

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
fn test_create_default_failure() {
    assert_that!(Factory::create("UnknownShape"), is(err()));
}

#[rstest]
fn test_create_with_circle() {
    let a_shape = Factory::create_with("Circle", &[5.0]).unwrap();
    let ref_shape = Circle::new(5.0);
    assert_that!(a_shape.to_string(), equal_to(ref_shape.to_string()));
}

fn test_create_with_square() {
    let a_shape = Factory::create_with("Square", &[5.0]).unwrap();
    let ref_shape = Square::new(5.0);
    assert_that!(a_shape.to_string(), equal_to(ref_shape.to_string()));
}

fn test_create_with_triangle() {
    let a_shape = Factory::create_with("Triangle", &[3.0, 4.0, 5.0]).unwrap();
    let ref_shape = Triangle::new(3.0, 4.0, 5.0);
    assert_that!(a_shape.to_string(), equal_to(ref_shape.to_string()));
}

fn test_create_with_right_triangle() {
    let a_shape = Factory::create_with("Right Triangle", &[3.0, 4.0]).unwrap();
    let ref_shape = RightTriangle::new(3.0, 4.0);
    assert_that!(a_shape.to_string(), equal_to(ref_shape.to_string()));
}

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
fn test_is_known(#[case] shape_name: &str) {
    assert!(Factory::is_known(shape_name));
}

#[rstest]
fn test_list_known_contains() {
    let f_str = Factory::list_known();

    assert!(f_str.contains("  Circle"));
    assert!(f_str.contains("  Square"));
    assert!(f_str.contains("  Triangle"));
    assert!(f_str.contains("  Right Triangle"));
    assert!(f_str.contains("  Equilateral Triangle"));
}

#[rstest]
fn test_number_known() {
    assert_that!(Factory::number_known(), is(equal_to(5)));
}
