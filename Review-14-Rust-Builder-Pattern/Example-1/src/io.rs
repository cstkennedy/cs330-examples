use itertools::Itertools;

use crate::flooring::FlooringBuilder;
use crate::house::{House, HouseBuilder};
use crate::room::{Room, RoomBuilder};
use std::vec::Vec;

pub fn read_house_from_str(room_data: &str) -> House {
    let parsed_rooms: Vec<Room> = room_data
        .lines()
        .filter(|line| line.len() > 0)
        .filter(|line| line.contains(";"))
        .map(|line| {
            // Grab the name first
            let line = line.split(";").collect::<Vec<&str>>();
            let name = line[0];

            // Split everything else by whitespace
            let the_rest: Vec<&str> = line[1].split_whitespace().collect();

            (name, the_rest)
        })
        .filter(|(_, the_rest)| the_rest.len() >= 4)
        .map(|(name, the_rest)| {
            // Parse the three f64 numbers
            let nums: Vec<f64> = the_rest[0..3]
                .iter()
                .map(|token| token.parse().unwrap_or(1_f64))
                .collect();

            // The flooring name might contain spaces.
            // Combine the remainder of the line.
            let flooring_name = the_rest.into_iter().skip(3).join(" ");

            (name, nums[0], nums[1], flooring_name, nums[2])
        })
        .map(|(name, length, width, flooring_name, unit_cost)| {
            RoomBuilder::new()
                .with_name(name)
                .with_dimensions(length, width)
                .with_flooring(
                    FlooringBuilder::new()
                        .with_specific_name(&flooring_name)
                        .with_unit_cost(unit_cost)
                        .build()
                        .unwrap(),
                )
                .build()
        })
        .flatten()
        .collect();

    // Create a house using the parsed rooms
    let house = HouseBuilder::new()
        .with_rooms(parsed_rooms)
        .build()
        .unwrap();

    house
}
