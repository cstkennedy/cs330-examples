use room_renovation::flooring::*;
use room_renovation::house::*;
use room_renovation::room::*;

use hamcrest2::prelude::*;

#[test]
fn test_cost() {
    todo!()
}

#[test]
fn test_defaults() {
    let flooring = Flooring::builder().build();

    assert_that!(&flooring.type_name, is(equal_to("Generic")));
    assert_that!(flooring.unit_cost, is(close_to(1.0, 0.001)));

    let reference = flooring;

    let flooring = Flooring::new();

    assert_that!(&flooring.type_name, is(equal_to("Generic")));
    assert_that!(flooring.unit_cost, is(close_to(1.0, 0.001)));
    assert_that!(&flooring, is(equal_to(&reference)));

    let flooring = Flooring::default();
    assert_that!(&flooring, is(equal_to(&reference)));
}

#[test]
fn test_with_values() {
    let flooring = Flooring::builder()
        .with_name("Stone Bricks")
        .with_unit_cost(12.97.try_into().unwrap())
        .build();

    assert_that!(&flooring.type_name, is(equal_to("Stone Bricks")));
    assert_that!(flooring.unit_cost, is(close_to(12.97, 0.001)));
}
