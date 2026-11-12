#[cfg(test)]
#[macro_use]
use hamcrest2::prelude::*;

use shapes::prelude::Factory;

use shapes::circle::Circle;
use shapes::equilateral_triangle::EquilateralTriangle;
use shapes::right_triangle::RightTriangle;
use shapes::square::Square;
use shapes::triangle::Triangle;

#[test]
fn test_is_known() {
    assert!(Factory::is_known("Circle"));
    assert!(Factory::is_known("Square"));
    assert!(Factory::is_known("Triangle"));
    assert!(Factory::is_known("Right Triangle"));
    assert!(Factory::is_known("Equilateral Triangle"));
}

#[test]
fn test_number_known() {
    assert_that!(Factory::number_known(), is(equal_to(5)));
}

#[test]
fn test_str() {
    let f_str = Factory::list_known();

    assert!(f_str.contains("  Circle"));
    assert!(f_str.contains("  Square"));
    assert!(f_str.contains("  Triangle"));
    assert!(f_str.contains("  Right Triangle"));
    assert!(f_str.contains("  Equilateral Triangle"));
}

#[test]
fn test_create() {
    // I need to write this test...
}

#[test]
fn test_create_with() {
    let a_shape = Factory::create_with("Triangle", &[3.0, 4.0, 5.0]).unwrap();
    let ref_shape = Triangle::new(3.0, 4.0, 5.0);
    assert_that!(a_shape.to_string(), equal_to(ref_shape.to_string()));

    let a_shape = Factory::create_with("Right Triangle", &[3.0, 4.0]).unwrap();
    let ref_shape = RightTriangle::new(3.0, 4.0);
    assert_that!(a_shape.to_string(), equal_to(ref_shape.to_string()));

    let a_shape = Factory::create_with("Equilateral Triangle", &[5.0]).unwrap();
    let ref_shape = EquilateralTriangle::new(5.0);
    assert_that!(a_shape.to_string(), equal_to(ref_shape.to_string()));

    let a_shape = Factory::create_with("Circle", &[5.0]).unwrap();
    let ref_shape = Circle::new(5.0);
    assert_that!(a_shape.to_string(), equal_to(ref_shape.to_string()));

    let a_shape = Factory::create_with("Square", &[5.0]).unwrap();
    let ref_shape = Square::new(5.0);
    assert_that!(a_shape.to_string(), equal_to(ref_shape.to_string()));
}
