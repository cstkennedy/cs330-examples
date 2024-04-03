#[cfg(test)]
#[macro_use]
use room_renovation::flooring::*;
use room_renovation::house::*;
use room_renovation::io;
use room_renovation::room::*;

use hamcrest2::prelude::*;

#[test]
fn test_empty_string() {
    let house = io::read_house_from_str("");

    assert_that!(house, is(none()));
}

#[test]
fn test_blank_string_white_space() {
    let house = io::read_house_from_str(" ");
    assert_that!(house, is(none()));

    let house = io::read_house_from_str(" \t ");
    assert_that!(house, is(none()));

    let line = [" ", " \t   ", "    \t\t", "\t"].join("\n");
    let house = io::read_house_from_str(&line);
    assert_that!(house, is(none()));
}

#[test]
fn test_malformed_lines_only_name() {
    let line = "Kitchen";
    let house = io::read_house_from_str(&line);
    assert_that!(house, is(none()));

    let lines = ["Kitchen", "", " Storage    "].join(";\n");
    let house = io::read_house_from_str(&lines);
    assert_that!(house, is(none()));
}

#[test]
fn test_malformed_lines_missing_tokens() {
    let line = ["", "4", "5", "7.5", "Vinyl Plank"].join(" ");
    let house = io::read_house_from_str(&line);
    assert_that!(house, is(none()));

    let line = ["Kitchen;", "", "5", "7.5", "Vinyl Plank"].join(" ");
    let house = io::read_house_from_str(&line);
    assert_that!(house, is(none()));

    let line = ["Kitchen;", "4", "", "7.5", "Vinyl Plank"].join(" ");
    let house = io::read_house_from_str(&line);
    assert_that!(house, is(none()));

    let line = ["Kitchen;", "4", "5", "", "Vinyl Plank"].join(" ");
    let house = io::read_house_from_str(&line);
    assert_that!(house, is(none()));

    let line = ["Kitchen;", "4", "5", "7.5", ""].join(" ");
    let house = io::read_house_from_str(&line);
    assert_that!(house, is(none()));

    let line = ["Kitchen;", "", "5", "", "Vinyl Plank"].join(" ");
    let house = io::read_house_from_str(&line);
    assert_that!(house, is(none()));

    let line = ["", "4", "5", "7.5", ""].join(" ");
    let house = io::read_house_from_str(&line);
    assert_that!(house, is(none()));
}

#[test]
fn test_one_room() {
    let line = "Kitchen; 4 5 7.5 Vinyl Plank";
    let house = io::read_house_from_str(&line);

    assert_that!(&house, is(some()));

    let house = house.unwrap();
    assert_that!(house.get_name(), is(equal_to("House")));
    assert_that!(house.len(), is(equal_to(1)));
}

#[test]
fn test_two_rooms() {
    let lines = [
        "Kitchen; 4 5 7.5 Vinyl Plank",
        "Storage Room; 2 4 7.5 Vinyl Plank"
    ].join("\n");

    let house = io::read_house_from_str(&lines);

    assert_that!(&house, is(some()));

    let house = house.unwrap();
    assert_that!(house.get_name(), is(equal_to("House")));
    assert_that!(house.len(), is(equal_to(2)));

    let expected_rooms = [
        Room::builder()
            .with_name("Kitchen")
            .with_dimensions(4.0, 5.0)
            .with_flooring(
                Flooring::builder()
                    .type_name("Vinyl Plank".to_owned())
                    .unit_cost(7.5)
                    .build()
            )
            .build()
            .unwrap(),
        Room::builder()
            .with_name("Storage Room")
            .with_dimensions(2.0, 4.0)
            .with_flooring(
                Flooring::builder()
                    .type_name("Vinyl Plank".to_owned())
                    .unit_cost(7.5)
                    .build()
            )
            .build()
            .unwrap()
    ];

    let num_matching_rooms: usize = expected_rooms
        .iter()
        .zip(house.iter())
        .filter(|(expected_room, actual_room)| &expected_room == &actual_room)
        .count();

    assert_that!(num_matching_rooms, is(equal_to(2)));
}

