use room_renovation::flooring::*;
use room_renovation::house::*;
use room_renovation::io::HouseParser;
use room_renovation::room::*;

use std::io::BufReader;

use hamcrest2::prelude::*;
use rstest::rstest;

#[rstest]
fn test_empty_string() {
    let str_reader = BufReader::new("".as_bytes());
    let house = HouseParser::read_house(str_reader);

    assert_that!(house, is(err()));
}

/*
#[rstest]
fn test_blank_string_white_space() {
    let house = HouseParser::read_house(" ");
    assert_that!(house, is(err()));

    let house = HouseParser::read_house(" \t ");
    assert_that!(house, is(err()));

    let line = [" ", " \t   ", "    \t\t", "\t"].join("\n");
    let house = HouseParser::read_house(line.as_str());
    assert_that!(house, is(err()));
}
*/

#[rstest]
fn test_malformed_lines_only_name() {
    let line = "Kitchen".as_bytes();
    let str_reader = BufReader::new(line);
    let house = HouseParser::read_house(str_reader);
    assert_that!(house, is(err()));

    let lines = ["Kitchen", "", " Storage    "].join(";\n");
    let str_reader = BufReader::new(lines.as_bytes());

    let house = HouseParser::read_house(str_reader);
    assert_that!(house, is(err()));
}

#[rstest]
#[case::missing_name(["", "4", "5", "7.5", "Vinyl Plank"])]
#[case::missing_length(["Kitchen;", "", "5", "7.5", "Vinyl Plank"])]
#[case::missing_width(["Kitchen;", "4", "", "7.5", "Vinyl Plank"])]
#[case::missing_cost(["Kitchen;", "4", "5", "", "Vinyl Plank"])]
#[case::missing_type(["Kitchen;", "4", "5", "7.5", ""])]
#[case(["Kitchen;", "", "5", "", "Vinyl Plank"])]
#[case(["", "4", "5", "7.5", ""])]
fn test_malformed_lines_missing_tokens(#[case] tokens: [&str; 5]) {
    let line = tokens.join(" ");
    let str_reader = BufReader::new(line.as_bytes());

    let house = HouseParser::read_house(str_reader);
    assert_that!(house, is(err()));
}

#[rstest]
fn test_one_room() {
    let line = "Kitchen; 4 5 7.5 Vinyl Plank";
    let str_reader = BufReader::new(line.as_bytes());

    let house = HouseParser::read_house(str_reader);

    assert_that!(&house, is(ok()));

    let house = house.unwrap();
    assert_that!(house.get_name(), is(equal_to("House")));
    assert_that!(house.len(), is(equal_to(1)));
}

#[rstest::fixture]
fn expected_rooms() -> [Room; 2] {
    [
        Room::builder()
            .with_name("Kitchen")
            .with_checked_dimensions((4.0, 5.0).try_into().unwrap())
            .with_flooring(
                Flooring::builder()
                    .with_name("Vinyl Plank")
                    .with_unit_cost(7.5.try_into().unwrap())
                    .build(),
            )
            .build(),
        Room::builder()
            .with_name("Storage Room")
            .with_checked_dimensions((2.0, 4.0).try_into().unwrap())
            .with_flooring(
                Flooring::builder()
                    .with_name("Vinyl Plank")
                    .with_unit_cost(7.5.try_into().unwrap())
                    .build(),
            )
            .build(),
    ]
}

#[rstest]
fn test_two_rooms(expected_rooms: [Room; 2]) {
    let lines = [
        "Kitchen; 4 5 7.5 Vinyl Plank",
        "Storage Room; 2 4 7.5 Vinyl Plank",
    ]
    .join("\n");
    let str_reader = BufReader::new(lines.as_bytes());

    let house = HouseParser::read_house(str_reader);

    assert_that!(&house, is(ok()));

    let house = house.unwrap();
    assert_that!(house.get_name(), is(equal_to("House")));
    assert_that!(house.len(), is(equal_to(2)));

    let num_matching_rooms: usize = expected_rooms
        .iter()
        .zip(house.iter())
        .filter(|(expected_room, actual_room)| &expected_room == &actual_room)
        .count();

    assert_that!(num_matching_rooms, is(equal_to(2)));
}
