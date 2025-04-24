#[cfg(test)]
#[macro_use]
extern crate hamcrest2;
extern crate shapes;

use hamcrest2::prelude::*;

use shapes::factory::Factory;

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
