use itertools::Itertools;

use crate::error::BuildError;
use crate::flooring::Flooring;
use crate::house::House;
use crate::room::Room;
use std::vec::Vec;

pub fn read_house_from_str(room_data: &str) -> Option<House> {
    let parsed_rooms: Vec<Room> = room_data
        .lines()
        .filter(|line| line.len() > 0)
        .filter(|line| line.contains(";"))
        .map(|line| {
            let line = line.split(";").collect::<Vec<&str>>();

            // Grab the name first
            let name = line[0];

            // Split everything else by whitespace
            let the_rest: Vec<&str> = line[1].split_whitespace().collect();

            (name, the_rest)
        })
        .filter(|(_, the_rest)| the_rest.len() >= 4)
        .map(|(name, the_rest)| {
            let nums: Vec<f64> = the_rest[0..3]
                .iter()
                .map(|token| token.parse())
                .flatten()
                .collect();

            // The flooring name might contain spaces.
            // Combine the remainder of the line.
            let flooring_name = the_rest.into_iter().skip(3).join(" ");

            (name, nums, flooring_name)
        })
        .filter(|(_, nums, _)| nums.len() == 3)
        .map(|(name, nums, flooring_name)| (name, nums[0], nums[1], flooring_name, nums[2]))
        .filter(|(_, length, width, _, unit_cost)| -> bool {
            return *length > 0.0 && *width > 0.0 && *unit_cost >= 0.01;
        })
        .map(|(name, length, width, flooring_name, unit_cost)| {
            Room::builder()
                .with_name(name)
                .with_dimensions(length, width)
                .with_flooring(
                    Flooring::builder()
                        .type_name(flooring_name)
                        .unit_cost(unit_cost)
                        .build(),
                )
                .build()
        })
        .flatten()
        .collect();

    let house = match House::builder().with_rooms(parsed_rooms) {
        Ok(builder) => builder.build().unwrap().into(),
        Err(_) => None,
    };

    house
}
