#[cfg(test)]
#[macro_use]
use room_renovation::flooring::*;
use room_renovation::house::*;
use room_renovation::room::*;

use hamcrest2::prelude::*;

#[test]
fn test_dimension_set() {
    let dims = DimensionSet::default();

    assert_that!(dims.length, is(close_to(1.0, 0.01)));
    assert_that!(dims.width, is(close_to(1.0, 0.01)));

    let dims = DimensionSet::new(2.0, 3.0);

    assert_that!(dims.length, is(close_to(2.0, 0.01)));
    assert_that!(dims.width, is(close_to(3.0, 0.01)));

    let dims: DimensionSet = (4.0, 5.0).into();

    assert_that!(dims.length, is(close_to(4.0, 0.01)));
    assert_that!(dims.width, is(close_to(5.0, 0.01)));
}

#[test]
fn test_from_existing_no_changes() {
    let original = Room::builder()
        .with_name("Kitchen")
        .with_dimensions(12.0, 20.0)
        .unwrap()
        .with_flooring(
            Flooring::builder()
                .type_name("Stone Bricks".into())
                .unit_cost(12.97)
                .build(),
        )
        .build();

    let duplicate = Room::builder().from_existing(&original).build();

    assert_that!(&duplicate, is(equal_to(&original)));
}

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
