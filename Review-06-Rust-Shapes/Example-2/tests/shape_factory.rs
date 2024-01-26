#[cfg(test)]
#[macro_use]
extern crate hamcrest2;
extern crate shapes;

use hamcrest2::prelude::*;

use shapes::factory::Factory;

#[test]
fn test_is_known() {
    let factory = Factory::new();

    assert!(factory.is_known("Circle"));
    assert!(factory.is_known("Square"));
    assert!(factory.is_known("Triangle"));
    assert!(factory.is_known("Right Triangle"));
    assert!(factory.is_known("Equilateral Triangle"));
}

#[test]
fn test_number_known() {
    let factory = Factory::new();

    assert_that!(factory.number_known(), is(equal_to(5)));
}

#[test]
fn test_str() {
    let factory = Factory::new();
    let f_str = factory.list_known();

    assert!(f_str.contains("  Circle"));
    assert!(f_str.contains("  Square"));
    assert!(f_str.contains("  Triangle"));
    assert!(f_str.contains("  Right Triangle"));
    assert!(f_str.contains("  Equilateral Triangle"));
}
