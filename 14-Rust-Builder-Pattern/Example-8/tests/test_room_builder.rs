use room_renovation::flooring::*;
use room_renovation::house::*;
use room_renovation::room::*;

use rstest::rstest;
use hamcrest2::prelude::*;

#[rstest]
fn test_dimension_set_default() {
    let dims = DimensionSet::default();

    assert_that!(dims.length, is(close_to(1.0, 0.01)));
    assert_that!(dims.width, is(close_to(1.0, 0.01)));
}

#[rstest]
#[case((2.0, 3.0))]
#[case((4.0, 5.0))]
fn test_dimension_set_ok(#[case] dims_tuple: (f64, f64)) {
    let dims: DimensionSet = dims_tuple.try_into().unwrap();

    assert_that!(dims.length, is(close_to(dims_tuple.0, 0.01)));
    assert_that!(dims.width, is(close_to(dims_tuple.1, 0.01)));
}

#[rstest]
#[case((-10.0, 1.0))]
#[case((0.0, 1.0))]
#[case((1.0, 0.0))]
fn test_dimension_set_err(#[case] dims_tuple: (f64, f64)) {
    assert_that!(TryInto::<DimensionSet>::try_into(dims_tuple), is(err()));
}

#[test]
fn test_from_existing_no_changes() {
    let original = Room::builder()
        .with_name("Kitchen")
        .with_checked_dimensions((12.0, 20.0).try_into().unwrap())
        .with_flooring(
            Flooring::builder()
                .with_name("Stone Bricks")
                .with_unit_cost(12.97.try_into().unwrap())
                .build(),
        )
        .build();

    let duplicate = Room::builder().from_existing(&original).build();

    assert_that!(&duplicate, is(equal_to(&original)));
}

/*
#[test]
fn test_from_existing_with_changes() {
    let original = Room::builder()
        .with_name("Kitchen")
        .with_dimensions(12.0, 20.0)
        .unwrap()
        .with_flooring(
            Flooring::builder()
                .type_name("Cherry Blossom Tile".into())
                .unit_cost(12.97)
                .build(),
        )
        .build();

    let tile = Flooring::builder()
        .type_name("Neutral Tile".into())
        .unit_cost(14.00)
        .build();

    let improved = Room::builder()
        .from_existing(&original)
        .with_dimensions(16.0, 20.0)
        .unwrap()
        .with_flooring(tile.clone())
        .build();

    assert_that!(&improved, is(not(equal_to(&original))));
    assert_that!(&improved, is(greater_than(&original)));

    assert_that!(&improved.flooring, is(equal_to(&tile)));
    assert_that!(improved.dimensions.length, is(close_to(16.0, 0.01)));
    assert_that!(improved.dimensions.width, is(close_to(20.0, 0.01)));
}
*/
