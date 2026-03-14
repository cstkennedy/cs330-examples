use room_renovation::flooring::*;
use room_renovation::house::*;
use room_renovation::room::*;

use hamcrest2::prelude::*;

#[test]
fn test_with_room_1() {
    let house = House::builder()
        .with_name("Test House")
        .with_room(Room::default())
        .build();

    assert_that!(house.get_name(), is(equal_to("Test House")));
    assert_that!(house.is_empty(), is(false));
    assert_that!(house.len(), is(equal_to(1)));
}

#[test]
fn test_with_room_2() {
    let house = House::builder()
        .with_name("Test House")
        .with_room(Room::default())
        .with_room(
            Room::builder()
                .with_name("Kitchen")
                .with_checked_dimensions((16.0, 20.0).try_into().unwrap())
                .with_flooring(
                    Flooring::builder()
                        .with_name("Neutral Tile")
                        .with_unit_cost(14.00.try_into().unwrap())
                        .build(),
                )
                .build(),
        )
        .build();

    assert_that!(house.get_name(), is(equal_to("Test House")));
    assert_that!(house.is_empty(), is(false));
    assert_that!(house.len(), is(equal_to(2)));
}

#[test]
fn test_empty_vec() {
    let builder_empty_vec = House::builder().with_name("Test House").with_rooms(vec![]);

    assert_that!(builder_empty_vec, err());
}

#[test]
fn test_empty_vec_then_with_room_1() {
    let house = House::builder()
        .with_name("Test House")
        .with_rooms(vec![])
        .err()
        .unwrap()
        .the_builder
        .with_room(Room::default())
        .build();

    assert_that!(house.get_name(), is(equal_to("Test House")));
    assert_that!(house.is_empty(), is(false));
    assert_that!(house.len(), is(equal_to(1)));
}

#[test]
fn test_empty_vec_then_with_room_2() {
    let kitchen = Room::builder()
        .with_name("Kitchen")
        .with_checked_dimensions((16.0, 20.0).try_into().unwrap())
        .with_flooring(
            Flooring::builder()
                .with_name("Neutral Tile")
                .with_unit_cost(14.00.try_into().unwrap())
                .build(),
        )
        .build();

    let house = House::builder()
        .with_name("Test House")
        .with_rooms(vec![])
        .err()
        .unwrap()
        .the_builder
        .with_room(Room::default())
        .with_room(kitchen.clone())
        .build();

    assert_that!(house.get_name(), is(equal_to("Test House")));
    assert_that!(house.is_empty(), is(false));
    assert_that!(house.len(), is(equal_to(2)));

    let room = house.iter().nth(1).unwrap();
    assert_that!(room, is(equal_to(&kitchen)));
}
