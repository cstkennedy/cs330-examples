#[cfg(test)]
#[macro_use]

use room_renovation::flooring::*;
use room_renovation::room::*;
use room_renovation::house::*;

use hamcrest2::prelude::*;

#[test]
fn test_with_room_1()
{
    let house = House::builder()
        .with_name("Test House")
        .with_room(Room::default())
        .build()
        .unwrap();

    assert_that!(house.get_name(), is(equal_to("Test House")));
    assert_that!(house.is_empty(), is(false));
    assert_that!(house.len(), is(equal_to(1)));
}

#[test]
fn test_with_room_2()
{
    let house = House::builder()
        .with_name("Test House")
        .with_room(Room::default())
        .with_room(
            Room::builder()
                .with_name("Kitchen")
                .with_dimensions(16.0, 20.0)
                .with_flooring(
                    Flooring::builder()
                        .type_name("Neutral Tile".into())
                        .unit_cost(14.00)
                        .build()
                )
                .build()
                .unwrap()
        )
        .build()
        .unwrap();

    assert_that!(house.get_name(), is(equal_to("Test House")));
    assert_that!(house.is_empty(), is(false));
    assert_that!(house.len(), is(equal_to(2)));
}

#[test]
fn test_empty_vec()
{
    let builder_with_name = House::builder()
        .with_name("Test House");

    let builder_empty_vec = House::builder()
        .with_name("Test House")
        .with_rooms(vec![]);

    assert_that!(Err(builder_with_name), is(equal_to(builder_empty_vec)));
}

fn test_empty_vec_then_with_room_1()
{
    let house = House::builder()
        .with_name("Test House")
        .with_rooms(vec![])
        .unwrap()
        .with_room(Room::default())
        .build()
        .unwrap();

    assert_that!(house.get_name(), is(equal_to("Test House")));
    assert_that!(house.is_empty(), is(false));
    assert_that!(house.len(), is(equal_to(1)));
}

fn test_empty_vec_then_with_room_2()
{
    let house = House::builder()
        .with_name("Test House")
        .with_rooms(vec![])
        .unwrap()
        .with_room(Room::default())
        .with_room(
            Room::builder()
                .with_name("Kitchen")
                .with_dimensions(16.0, 20.0)
                .with_flooring(
                    Flooring::builder()
                        .type_name("Neutral Tile".into())
                        .unit_cost(14.00)
                        .build()
                )
                .build()
                .unwrap()
        )
        .build()
        .unwrap();

    assert_that!(house.get_name(), is(equal_to("Test House")));
    assert_that!(house.is_empty(), is(false));
    assert_that!(house.len(), is(equal_to(2)));
}

