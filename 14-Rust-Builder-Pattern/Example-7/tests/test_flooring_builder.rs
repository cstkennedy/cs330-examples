use room_renovation::flooring::*;
use room_renovation::house::*;
use room_renovation::room::*;

use rstest::rstest;
use hamcrest2::prelude::*;

#[rstest]
#[case(0.0)]
#[case(f64::MIN_POSITIVE)]
#[case(0.01)]
#[case(0.10)]
#[case(1.10)]
#[case(f64::MAX)]
fn test_cost_ok(#[case] raw_cost: f64) {
    assert_that!(Cost::try_from(raw_cost), is(ok()))
}

#[rstest]
fn test_cost_err() {
    todo!()
}

#[rstest]
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
