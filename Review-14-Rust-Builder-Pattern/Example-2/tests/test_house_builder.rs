#[cfg(test)]
#[macro_use]

use room_renovation::flooring::*;
use room_renovation::room::*;
use room_renovation::house::*;

use hamcrest2::prelude::*;

#[test]
fn test_first_room()
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
fn test_empty_vec()
{
    let builder_with_name = House::builder()
        .with_name("Test House");

    let builder_empty_vec = House::builder()
        .with_name("Test House")
        .with_rooms(vec![]);

    assert_that!(Err(builder_with_name), is(equal_to(builder_empty_vec)));
}
